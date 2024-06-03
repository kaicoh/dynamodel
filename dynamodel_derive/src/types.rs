use super::{case::RenameRule, utils::*};
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

#[derive(Debug)]
pub struct NamedField {
    name: String,
    field: Field,
}

impl NamedField {
    fn ident(&self) -> &Option<syn::Ident> {
        &self.field.ident
    }

    fn ty(&self) -> &syn::Type {
        &self.field.ty
    }

    fn attr_into(&self) -> Option<&syn::Expr> {
        self.field.into.as_ref()
    }

    fn attr_try_from(&self) -> Option<&syn::Expr> {
        self.field.try_from.as_ref()
    }

    fn attr_try_from_item(&self) -> Option<&syn::Expr> {
        self.field.try_from_item.as_ref()
    }

    pub fn skip_into(&self) -> bool {
        self.field.skip_into.as_ref().is_some_and(|v| *v)
    }

    pub fn set_named_field_token(&self) -> TokenStream {
        let field_name = self.ident();
        let ty = self.ty();
        let hash_key = self.name.as_str();

        if let Some(f) = self.attr_try_from_item() {
            return quote! { #field_name: #f(&item)? };
        }

        let get_value = quote! { item.get(#hash_key) };
        let field_not_set = not_set_err(field_name);

        if let Some(f) = self.attr_try_from() {
            return quote! {
                #field_name: #get_value
                    .ok_or(#field_not_set)
                    .and_then(#f)?
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

    pub fn set_key_value_pair_token<T>(&self, get_value: T) -> TokenStream
    where
        T: Fn(&Option<syn::Ident>) -> TokenStream,
    {
        let field_name = self.ident();
        let ty = self.ty();
        let name = self.name.as_str();
        let hash_key = quote! { #name.into() };

        let get_value_token = get_value(field_name);

        if let Some(f) = self.attr_into() {
            return quote! {
                let v = #get_value_token;
                item.insert(#hash_key, #f(v));
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

    pub fn into_named(self, rule: &RenameRule) -> NamedField {
        let name = self.rename.clone().unwrap_or_else(|| {
            let ident_str = self.ident.to_token_stream().to_string();
            rule.apply_to_field(&ident_str)
        });

        NamedField { name, field: self }
    }
}

#[derive(Debug)]
pub struct NamedVariant {
    name: String,
    variant: Variant,
}

impl NamedVariant {
    fn ident(&self) -> &syn::Ident {
        &self.variant.ident
    }

    fn assert_newtype(&self) {
        assert!(
            self.is_newtype(),
            "this method must be called on newtype variant."
        );
    }

    fn fields(&self) -> Vec<NamedField> {
        let rule = self
            .variant
            .rename_all
            .as_ref()
            .map(RenameRule::from_lit)
            .unwrap_or_default();

        self.variant
            .fields
            .fields
            .clone()
            .into_iter()
            .map(|f| f.into_named(&rule))
            .collect()
    }

    fn set_key_value_branch(&self, return_token: TokenStream) -> TokenStream {
        let ident = self.ident();

        let fields = self.fields();
        let field_names = fields.iter().map(NamedField::ident);
        let set_key_values = fields
            .iter()
            .map(|f| f.set_key_value_pair_token(|v| quote!(#v)));

        quote! {
            #ident { #(#field_names,)* } => {
                let mut item: Self = ::std::collections::HashMap::new();
                #(#set_key_values)*
                #return_token
            }
        }
    }

    fn is_newtype(&self) -> bool {
        self.variant.fields.is_newtype()
    }

    fn newtype_value_token(&self) -> TokenStream {
        self.assert_newtype();

        let ident = self.ident();
        let hash_key = self.name.as_str();

        let fields = self.fields();
        let ty = fields[0].ty();

        if inner_type_of("Option", ty).is_some() {
            unreachable!("Variant.validate must be called before this method");
        }

        let into_value = from_attribute_value(ty);

        quote! {
            if let Some(v) = item
                .get(#hash_key)
                .map(|v| #into_value)
                .transpose()?
            {
                return Ok(Self::#ident(v));
            }
        }
    }

    fn newtype_value_token_tagged(&self) -> TokenStream {
        self.assert_newtype();

        let ident = self.ident();
        let name = self.name.as_str();

        quote! {
            #name => {
                return Ok(Self::#ident(item.try_into()?));
            }
        }
    }

    fn named_value_token(&self) -> TokenStream {
        let ident = self.ident();
        let hash_key = self.name.as_str();
        let err = unmatch_err("M");

        let fields = self.fields();
        let fields_token = fields.iter().map(NamedField::set_named_field_token);

        quote! {
            if let Some(ref item) = item
                .get(#hash_key)
                .map(|v| v.as_m().map_err(|e| #err))
                .transpose()?
            {
                return Ok(Self::#ident { #(#fields_token,)* });
            }
        }
    }

    fn named_value_token_tagged(&self) -> TokenStream {
        let ident = self.ident();
        let name = self.name.as_str();

        let fields = self.fields();
        let fields_token = fields.iter().map(NamedField::set_named_field_token);

        quote! {
            #name => {
                return Ok(Self::#ident { #(#fields_token,)* });
            }
        }
    }

    fn set_newtype_key_value(&self) -> TokenStream {
        self.assert_newtype();

        let ident = self.ident();
        let name = self.name.as_str();

        let fields = self.fields();
        let ty = fields[0].ty();
        let value = into_attribute_value(ty);

        quote! {
            #ident(v) => {
                [(#name.into(), ::aws_sdk_dynamodb::types::AttributeValue::#value)].into()
            }
        }
    }

    fn set_tagged_newtype_key_value(&self, tag: &str) -> TokenStream {
        self.assert_newtype();

        let ident = self.ident();
        let name = self.name.as_str();

        quote! {
            #ident(v) => {
                let mut item: Self = v.into();
                item.insert(
                    #tag.into(),
                    ::aws_sdk_dynamodb::types::AttributeValue::S(#name.into()),
                );
                item
            }
        }
    }

    fn set_named_key_value(&self) -> TokenStream {
        let name = self.name.as_str();
        let return_token = quote! {
            [(#name.into(), ::aws_sdk_dynamodb::types::AttributeValue::M(item))].into()
        };

        self.set_key_value_branch(return_token)
    }

    fn set_tagged_named_key_value(&self, tag: &str) -> TokenStream {
        let name = self.name.as_str();

        let return_token = quote! {
            item.insert(
                #tag.into(),
                ::aws_sdk_dynamodb::types::AttributeValue::S(#name.into()),
            );
            item
        };

        self.set_key_value_branch(return_token)
    }

    pub fn get_value_token(&self) -> TokenStream {
        if self.is_newtype() {
            self.newtype_value_token()
        } else {
            self.named_value_token()
        }
    }

    pub fn get_value_token_tagged(&self) -> TokenStream {
        if self.is_newtype() {
            self.newtype_value_token_tagged()
        } else {
            self.named_value_token_tagged()
        }
    }

    pub fn set_key_value(&self) -> TokenStream {
        if self.is_newtype() {
            self.set_newtype_key_value()
        } else {
            self.set_named_key_value()
        }
    }

    pub fn set_tagged_key_value(&self, tag: &str) -> TokenStream {
        if self.is_newtype() {
            self.set_tagged_newtype_key_value(tag)
        } else {
            self.set_tagged_named_key_value(tag)
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
        if self.fields.is_newtype() {
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

    pub fn into_named(self, rule: &RenameRule) -> NamedVariant {
        let name = self.rename.clone().unwrap_or_else(|| {
            let ident_str = self.ident.to_token_stream().to_string();
            rule.apply_to_variant(&ident_str)
        });

        NamedVariant {
            name,
            variant: self,
        }
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
