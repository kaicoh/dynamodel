mod case;
mod getter;
mod setter;
mod utils;

use case::RenameRule;
use darling::{util::WithOriginal, FromDeriveInput};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

// The main struct we get from parsing the attributes
// The "supports(struct_named)" attribute guarantees only named structs to work with this macro
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dynamodel), supports(struct_named))]
#[darling(and_then = "TargetStruct::validate")]
struct TargetStruct {
    ident: syn::Ident,
    data: darling::ast::Data<(), WithOriginal<utils::Field, syn::Field>>,
    rename_all: Option<syn::Expr>,
    extra: Option<darling::Result<syn::Path>>,
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

    fn token_stream(self) -> TokenStream {
        let ident = self.ident;
        let fields = self.data.take_struct().unwrap().fields;

        let rename_rule = self.rename_all.as_ref().map(RenameRule::new);

        let setters = fields.iter().filter_map(|f| {
            if f.parsed.skip_into.is_some_and(|v| v) {
                None
            } else {
                Some(setter::token_stream(&f.parsed, &rename_rule))
            }
        });

        let getters = fields
            .iter()
            .map(|f| getter::token_stream(&f.parsed, &rename_rule));

        let set_extra = if self.extra.is_some() {
            quote! { item.extend(key); }
        } else {
            quote!()
        };

        let init_extra = match self.extra {
            Some(Ok(path)) => {
                quote! {
                    let key: Self = #path(&value);
                }
            }
            Some(Err(err)) => {
                abort! {
                    err.span(), "Invalid attribute #[dynamodel(extra = ...)]";
                    note = "Invalid argument for `extra` attribute. Only paths are allowed.";
                    help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`";
                }
            }
            None => quote!(),
        };

        quote! {
            impl ::std::convert::From<#ident> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> {
                fn from(value: #ident) -> Self {
                    #init_extra

                    let mut item: Self = ::std::collections::HashMap::new();
                    #(#setters)*

                    #set_extra

                    item
                }
            }

            impl ::std::convert::TryFrom<::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>> for #ident {
                type Error = ::dynamodel::ConvertError;

                fn try_from(item: ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#getters,)*
                    })
                }
            }
        }.into()
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
