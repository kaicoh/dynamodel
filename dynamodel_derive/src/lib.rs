mod case;
mod getter;
mod setter;
mod utils;

use case::RenameRule;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// The main struct we get from parsing the attributes
// The "supports(struct_named)" attribute guarantees only named structs to work with this macro
// Ref: https://github.com/TedDriggs/darling?tab=readme-ov-file#shape-validation
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dynamodel), supports(struct_named, enum_named))]
struct TargetStruct {
    ident: syn::Ident,
    data: darling::ast::Data<(), utils::Field>,
    rename_all: Option<syn::Expr>,
    extra: Option<darling::Result<syn::Path>>,
}

impl TargetStruct {
    fn token_stream(self) -> TokenStream {
        let ident = self.ident;
        let fields = self.data.take_struct().unwrap().fields;

        let rename_rule = self.rename_all.as_ref().map(RenameRule::new);

        let setters = fields.iter().map(|f| setter::token_stream(f, &rename_rule));
        let getters = fields.iter().map(|f| getter::token_stream(f, &rename_rule));

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
