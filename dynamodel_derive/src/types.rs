use super::{case::RenameRule, utils::*};
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromField, Clone)]
#[darling(attributes(dynamodel))]
pub struct Field {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub attrs: Vec<syn::Attribute>,
    pub into: Option<syn::Expr>,
    pub try_from: Option<syn::Expr>,
    pub rename: Option<String>,
    pub skip_into: Option<bool>,
    pub try_from_item: Option<syn::Expr>,
}

impl Field {
    pub fn renamed(&self, rule: &RenameRule) -> TokenStream {
        let field_name = &self.ident;

        if let Some(ref renamed_field) = self.rename {
            return token_from_str(renamed_field);
        }

        let field_name_str = field_name.to_token_stream().to_string();
        let renamed_field = rule.apply_to_field(&field_name_str);
        token_from_str(&renamed_field)
    }

    // token to retrieve field value from the HashMap
    pub fn getter(&self, rule: &RenameRule) -> TokenStream {
        let field_name = &self.ident;
        let ty = &self.ty;
        let field_not_set = not_set_err(field_name);
        let hash_key = self.renamed(rule);

        if let Some(ref converter) = self.try_from_item {
            return quote! {
                #field_name: #converter(&item)?
            };
        }

        let get_value = quote! { item.get(stringify!(#hash_key)) };

        if let Some(ref converter) = self.try_from {
            return quote! {
                #field_name: #get_value
                    .ok_or(#field_not_set)
                    .and_then(#converter)?
            };
        }

        match inner_type_of("Option", ty) {
            Some(ty) => {
                let into_value = from_attribute_value(ty);

                quote! {
                    #field_name: #get_value
                        .map(|v| { #into_value })
                        .transpose()?
                }
            }
            None => {
                let into_value = from_attribute_value(ty);

                quote! {
                    #field_name: #get_value
                        .ok_or(#field_not_set)
                        .and_then(|v| { #into_value })?
                }
            }
        }
    }

    // token to set key, value pair to the HashMap.
    pub fn setter<T>(&self, rule: &RenameRule, get_value: T) -> TokenStream
    where
        T: Fn(&Option<syn::Ident>) -> TokenStream,
    {
        let field_name = &self.ident;
        let ty = &self.ty;
        let hash_key_token = self.renamed(rule);
        let hash_key = quote! { stringify!(#hash_key_token).into() };

        let get_value_token = get_value(field_name);

        if let Some(ref converter) = self.into {
            return quote! {
                let v = #get_value_token;
                item.insert(#hash_key, #converter(v));
            };
        }

        match inner_type_of("Option", ty) {
            Some(ty) => {
                let variant = into_attribute_value(ty);

                quote! {
                    if let Some(v) = #get_value_token {
                        item.insert(
                            #hash_key,
                            ::aws_sdk_dynamodb::types::AttributeValue::#variant,
                        );
                    }
                }
            }
            None => {
                let variant = into_attribute_value(ty);

                quote! {
                    let v = #get_value_token;
                    item.insert(
                        #hash_key,
                        ::aws_sdk_dynamodb::types::AttributeValue::#variant,
                    );
                }
            }
        }
    }
}

#[derive(Debug, FromVariant, Clone)]
#[darling(attributes(dynamodel))]
pub struct Variant {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,
    pub fields: darling::ast::Fields<Field>,
    pub rename: Option<String>,
    pub rename_all: Option<syn::Lit>,
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens)
    }
}

impl Variant {
    pub fn renamed(&self, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;

        if let Some(ref renamed_variant) = self.rename {
            return token_from_str(renamed_variant);
        }

        let name_str = name.to_token_stream().to_string();
        let renamed = rule.apply_to_variant(&name_str);
        token_from_str(&renamed)
    }

    pub fn getter_branch(&self, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;
        let fields = &self.fields.fields;
        let field_rename_rule = self.rename_rule_for_fields();
        let renamed = self.renamed(rule);

        let field_getters = fields.iter().map(|f| f.getter(&field_rename_rule));

        quote! {
            stringify!(#renamed) => {
                return Ok(Self::#name { #(#field_getters,)* });
            }
        }
    }

    pub fn setter_branch(&self, tag: &Option<String>, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;
        let field_names = self.fields.fields.iter().map(|f| &f.ident);

        let field_setters = self.field_setters();
        let variant_setter = self.variant_setter(tag, rule);

        quote! {
            #name { #(#field_names,)* } => {
                #(#field_setters)*
                #variant_setter
            }
        }
    }

    fn rename_rule_for_fields(&self) -> RenameRule {
        self.rename_all
            .as_ref()
            .map(RenameRule::from_lit)
            .unwrap_or_default()
    }

    fn variant_setter(&self, tag: &Option<String>, rule: &RenameRule) -> TokenStream {
        let renamed_variant = self.renamed(rule);
        let renamed = quote! { stringify!(#renamed_variant).into() };

        if let Some(ref tag) = tag {
            let tag = token_from_str(tag);

            quote! {
                item.insert(
                    stringify!(#tag).into(),
                    ::aws_sdk_dynamodb::types::AttributeValue::S(#renamed),
                );
            }
        } else {
            quote! {
                outer.insert(
                    #renamed,
                    ::aws_sdk_dynamodb::types::AttributeValue::M(item),
                );
            }
        }
    }

    fn field_setters(&self) -> impl Iterator<Item = TokenStream> + '_ {
        let fields = &self.fields.fields;
        let field_rename_rule = self.rename_rule_for_fields();

        fields
            .iter()
            .map(move |f| f.setter(&field_rename_rule, |v| quote!(#v)))
    }
}

fn into_attribute_value(ty: &syn::Type) -> TokenStream {
    if is_string(ty) {
        return quote! { S(v) };
    }

    if is_bool(ty) {
        return quote! { Bool(v) };
    }

    if is_number(ty) {
        return quote! { N(v.to_string()) };
    }

    if is_string_vec(ty) {
        return quote! {
            L(v.into_iter()
              .map(::aws_sdk_dynamodb::types::AttributeValue::S)
              .collect())
        };
    }

    if is_bool_vec(ty) {
        return quote! {
            L(v.into_iter()
              .map(::aws_sdk_dynamodb::types::AttributeValue::Bool)
              .collect())
        };
    }

    if is_number_vec(ty) {
        return quote! {
            L(v.into_iter()
              .map(|v| ::aws_sdk_dynamodb::types::AttributeValue::N(v.to_string()))
              .collect())
        };
    }

    if is_any_vec(ty) {
        return quote! {
            L(v.into_iter()
              .map(|v| ::aws_sdk_dynamodb::types::AttributeValue::M(v.into()))
              .collect())
        };
    }

    quote! { M(v.into()) }
}

fn from_attribute_value(ty: &syn::Type) -> TokenStream {
    if is_string(ty) {
        let err = unmatch_err("S");
        return quote! {
            v.as_s().map(|val| val.clone()).map_err(|e| #err)
        };
    }

    if is_bool(ty) {
        let err = unmatch_err("Bool");
        return quote! {
            v.as_bool().map(|val| *val).map_err(|e| #err)
        };
    }

    if is_number(ty) {
        let err = unmatch_err("N");
        return quote! {
            v.as_n().map_err(|e| #err)
                .and_then(|val| val.parse::<#ty>().map_err(|e| e.into()))
        };
    }

    if is_string_vec(ty) {
        let err = unmatch_err("S");
        return l_wrapper(
            &quote!(String),
            quote! {
                match i.as_s() {
                    Ok(v) => {
                        values.push(v.clone());
                    }
                    Err(e) => {
                        return Err(#err);
                    }
                }
            },
        );
    }

    if is_bool_vec(ty) {
        let err = unmatch_err("Bool");
        return l_wrapper(
            &quote!(bool),
            quote! {
                match i.as_bool() {
                    Ok(v) => {
                        values.push(*v);
                    }
                    Err(e) => {
                        return Err(#err);
                    }
                }
            },
        );
    }

    if is_number_vec(ty) {
        let err = unmatch_err("N");
        if let Some(inner_ty) = inner_type(ty) {
            return l_wrapper(
                inner_ty,
                quote! {
                    match i.as_n().map(|val| val.parse::<#inner_ty>()) {
                        Ok(Ok(v)) => {
                            values.push(v);
                        }
                        Ok(Err(e)) => {
                            return Err(e.into());
                        }
                        Err(e) => {
                            return Err(#err);
                        }
                    }
                },
            );
        } else {
            unreachable!("expect a vector of number, got {ty:#?}");
        }
    }

    if is_any_vec(ty) {
        let err = unmatch_err("M");
        if let Some(inner_ty) = inner_type(ty) {
            return l_wrapper(
                inner_ty,
                quote! {
                    match i.as_m().map(|val| #inner_ty::try_from(val.clone())) {
                        Ok(Ok(v)) => {
                            values.push(v);
                        }
                        Ok(Err(e)) => {
                            return Err(e);
                        }
                        Err(e) => {
                            return Err(#err);
                        }
                    }
                },
            );
        } else {
            unreachable!("expect a vector of model implementing TryFrom<HashMap<String, AttributeValue>>, got {ty:#?}");
        }
    }

    let err = unmatch_err("M");
    quote! {
        v.as_m().map_err(|e| #err)
            .and_then(|val| #ty::try_from(val.clone()))
    }
}

fn not_set_err(ident: &Option<syn::Ident>) -> TokenStream {
    quote! {
        ::dynamodel::ConvertError::FieldNotSet(stringify!(#ident).into())
    }
}

fn unmatch_err(ty: &str) -> TokenStream {
    quote! {
        ::dynamodel::ConvertError::AttributeValueUnmatched(#ty.into(), e.clone())
    }
}

fn l_wrapper(ty: &impl ToTokens, token: TokenStream) -> TokenStream {
    let l_err = unmatch_err("L");
    quote! {
        v.as_l().map_err(|e| #l_err).and_then(|l| {
            let mut values: Vec<#ty> = vec![];
            for i in l.iter() { #token }
            Ok(values)
        })
    }
}
