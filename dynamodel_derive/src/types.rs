use super::{case::RenameRule, utils::*};
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

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
    pub fn validate(&self) {
        if self.try_from.is_some() && self.try_from_item.is_some() {
            abort! {
                self.try_from.clone().unwrap().span(), "Invalid attribute #[dynamodel(try_from = ..., try_from_item = ...)]";
                note = "Either `try_from` or `try_from_item` can be set.";
                help = "Try removing either `try_from` or `try_from_item`.";
            }
        }
    }

    fn renamed(&self, rule: &RenameRule) -> TokenStream {
        let field_name = &self.ident;

        if let Some(ref renamed_field) = self.rename {
            return token_from_str(renamed_field);
        }

        let field_name_str = field_name.to_token_stream().to_string();
        let renamed_field = rule.apply_to_field(&field_name_str);
        token_from_str(&renamed_field)
    }

    pub fn set_named_field_token(&self, rule: &RenameRule) -> TokenStream {
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
    pub fn named_setter<T>(&self, rule: &RenameRule, get_value: T) -> TokenStream
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
    pub fn validate(&self) {
        if self.is_newtype() {
            let ty = &self.fields.fields[0].ty;

            if inner_type_of("Option", ty).is_some() {
                abort! {
                    ty.span(), "newtype variant with optional is not supported.";
                    note = "You cannot use tagged newtype variant containing an optional.";
                }
            }
        } else {
            for field in self.fields.fields.iter() {
                field.validate();
            }
        }
    }

    pub fn renamed(&self, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;

        if let Some(ref renamed_variant) = self.rename {
            return token_from_str(renamed_variant);
        }

        let name_str = name.to_token_stream().to_string();
        let renamed = rule.apply_to_variant(&name_str);
        token_from_str(&renamed)
    }

    pub fn is_newtype(&self) -> bool {
        self.fields.is_newtype()
    }

    pub fn newtype_value_token(&self, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;
        let renamed = self.renamed(rule);

        let ty = &self.fields.fields[0].ty;

        if inner_type_of("Option", ty).is_some() {
            unreachable!("Variant.validate must be called before this method");
        }

        let into_value = from_attribute_value(ty);
        let mapping = quote! { map(|v| #into_value).transpose()? };

        quote! {
            if let Some(v) = item.get(stringify!(#renamed)).#mapping {
                return Ok(Self::#name(v));
            }
        }
    }

    pub fn newtype_value_token_tagged(&self, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;
        let renamed = self.renamed(rule);

        quote! {
            stringify!(#renamed) => {
                return Ok(Self::#name(item.try_into()?));
            }
        }
    }

    pub fn named_value_token(&self, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;
        let renamed = self.renamed(rule);

        let rule = self.rename_rule_for_fields();
        let fields_token = self
            .fields
            .fields
            .iter()
            .map(|f| f.set_named_field_token(&rule));

        quote! {
            stringify!(#renamed) => {
                return Ok(Self::#name { #(#fields_token,)* });
            }
        }
    }

    fn newtype_setter(&self, tag: &Option<String>, rule: &RenameRule) -> TokenStream {
        let renamed_variant = self.renamed(rule);
        let renamed = quote! { stringify!(#renamed_variant).into() };

        let name = &self.ident;
        let ty = &self.fields.fields[0].ty;
        let value = into_attribute_value(ty);

        if let Some(ref tag) = tag {
            let tag = token_from_str(tag);

            quote! {
                #name(v) => {
                    let mut item: Self = v.into();
                    item.insert(
                        stringify!(#tag).into(),
                        ::aws_sdk_dynamodb::types::AttributeValue::S(#renamed),
                    );
                    item
                }
            }
        } else {
            quote! {
                #name(v) => {
                    [(#renamed, ::aws_sdk_dynamodb::types::AttributeValue::#value)].into()
                }
            }
        }
    }

    fn named_setters(&self, tag: &Option<String>, rule: &RenameRule) -> TokenStream {
        let name = &self.ident;
        let field_names = self.fields.fields.iter().map(|f| &f.ident);

        let field_setters = self.field_setters();
        let return_value = self.return_hashmap(tag, rule);

        quote! {
            #name { #(#field_names,)* } => {
                let mut item: Self = ::std::collections::HashMap::new();
                #(#field_setters)*
                #return_value
            }
        }
    }

    pub fn setters(&self, tag: &Option<String>, rule: &RenameRule) -> TokenStream {
        if self.is_newtype() {
            self.newtype_setter(tag, rule)
        } else {
            self.named_setters(tag, rule)
        }
    }

    fn rename_rule_for_fields(&self) -> RenameRule {
        self.rename_all
            .as_ref()
            .map(RenameRule::from_lit)
            .unwrap_or_default()
    }

    fn return_hashmap(&self, tag: &Option<String>, rule: &RenameRule) -> TokenStream {
        let renamed_variant = self.renamed(rule);
        let renamed = quote! { stringify!(#renamed_variant).into() };

        if let Some(ref tag) = tag {
            let tag = token_from_str(tag);

            quote! {
                item.insert(
                    stringify!(#tag).into(),
                    ::aws_sdk_dynamodb::types::AttributeValue::S(#renamed),
                );
                item
            }
        } else {
            quote! {
                [(#renamed, ::aws_sdk_dynamodb::types::AttributeValue::M(item))].into()
            }
        }
    }

    fn field_setters(&self) -> impl Iterator<Item = TokenStream> + '_ {
        let fields = &self.fields.fields;
        let field_rename_rule = self.rename_rule_for_fields();

        fields
            .iter()
            .map(move |f| f.named_setter(&field_rename_rule, |v| quote!(#v)))
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
