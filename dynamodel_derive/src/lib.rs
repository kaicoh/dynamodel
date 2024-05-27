mod case;
mod impls;
mod types;
mod utils;

use case::RenameRule;
use darling::{util::WithOriginal, FromDeriveInput};
use impls::{enums, structs};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

// The main struct we get from parsing the attributes
// Ref: https://github.com/TedDriggs/darling?tab=readme-ov-file#shape-validation
#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(dynamodel),
    supports(struct_named, enum_named, enum_newtype)
)]
#[darling(and_then = "TargetStruct::validate")]
struct TargetStruct {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<types::Variant, WithOriginal<types::Field, syn::Field>>,
    rename_all: Option<syn::Lit>,
    extra: Option<darling::Result<syn::Path>>,
    tag: Option<String>,
}

impl TargetStruct {
    fn validate(self) -> darling::Result<Self> {
        if let darling::ast::Data::Struct(fields) = &self.data {
            if let Some(f) = fields
                .fields
                .iter()
                .find(|f| f.parsed.try_from.is_some() && f.parsed.try_from_item.is_some())
            {
                abort! {
                    f.original.span(), "Invalid attribute #[dynamodel(try_from = ..., try_from_item = ...)]";
                    note = "Either `try_from` or `try_from_item` can be set.";
                    help = "Try removing either `try_from` or `try_from_item`.";
                }
            }
        }
        Ok(self)
    }

    fn rename_rule(&self) -> RenameRule {
        self.rename_all
            .as_ref()
            .map(RenameRule::from_lit)
            .unwrap_or_default()
    }

    fn token_stream_struct(self) -> TokenStream {
        let ident = &self.ident;
        let (imp, ty, whr) = self.generics.split_for_impl();
        let rename_rule = self.rename_rule();

        let fields: Vec<types::Field> = self
            .data
            .take_struct()
            .unwrap()
            .fields
            .into_iter()
            .map(|f| f.parsed)
            .collect();

        let into_hashmap =
            structs::into_hashmap(ident, &self.tag, &self.extra, &fields, &rename_rule);
        let try_from_hashmap = structs::try_from_hashmap(&fields, &rename_rule);

        quote! {
            impl #imp ::std::convert::From<#ident #ty> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> #whr {
                fn from(value: #ident #ty) -> Self {
                    #into_hashmap
                }
            }

            impl #imp ::std::convert::TryFrom<::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>> for #ident #ty #whr {
                type Error = ::dynamodel::ConvertError;

                fn try_from(item: ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                    #try_from_hashmap
                }
            }
        }.into()
    }

    fn token_stream_enum(self) -> TokenStream {
        let ident = &self.ident;
        let (imp, ty, whr) = self.generics.split_for_impl();
        let rename_rule = self.rename_rule();
        let variants = self.data.take_enum().unwrap();

        let into_hashmap = enums::into_hashmap(ident, &self.tag, &variants, &rename_rule);
        let try_from_hashmap = enums::try_from_hashmap(&self.tag, &variants, &rename_rule);

        quote! {
            impl #imp ::std::convert::From<#ident #ty> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> #whr {
                fn from(value: #ident #ty) -> Self {
                    #into_hashmap
                }
            }

            impl #imp ::std::convert::TryFrom<::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>> for #ident #ty #whr {
                type Error = ::dynamodel::ConvertError;

                fn try_from(item: ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                    #try_from_hashmap
                }
            }
        }.into()
    }

    fn token_stream(self) -> TokenStream {
        match self.data {
            darling::ast::Data::Struct(_) => self.token_stream_struct(),
            darling::ast::Data::Enum(_) => self.token_stream_enum(),
        }
    }
}

#[proc_macro_error]
#[proc_macro_derive(Dynamodel, attributes(dynamodel))]
pub fn derive_dynamodel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    TargetStruct::from_derive_input(&input)
        .map(TargetStruct::token_stream)
        .unwrap_or_else(|e| e.write_errors().into())
}
