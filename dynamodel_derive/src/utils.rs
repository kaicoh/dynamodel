use super::case::RenameRule;
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn token_from_str(value: &String) -> TokenStream {
    value.to_owned().parse().unwrap()
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
    pub fn hash_key(&self, rule: &Option<RenameRule>) -> TokenStream {
        let field_name = &self.ident;

        if let Some(ref renamed_field) = self.rename {
            return token_from_str(renamed_field);
        }

        if let Some(ref rule) = rule {
            let field_name_str = field_name.to_token_stream().to_string();
            let renamed_field = rule.apply(&field_name_str);
            token_from_str(&renamed_field)
        } else {
            quote!(#field_name)
        }
    }
}

#[derive(Debug, FromVariant, Clone)]
#[darling(attributes(dynamodel))]
pub struct Variant {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,
    pub fields: darling::ast::Fields<Field>,
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens)
    }
}
