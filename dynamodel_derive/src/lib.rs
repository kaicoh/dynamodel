mod getter;
mod setter;
mod utils;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// The main struct we get from parsing the attributes
// The "supports(struct_named)" attribute guarantees only named structs to work with this macro
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dynamodel), supports(struct_named))]
struct TargetStruct {
    ident: syn::Ident,
    data: darling::ast::Data<(), utils::Field>,
}

impl TargetStruct {
    fn token_stream(self) -> TokenStream {
        let ident = self.ident;
        let fields = self.data.take_struct().unwrap().fields;

        let setters = fields.iter().map(setter::token_stream);
        let getters = fields.iter().map(getter::token_stream);

        quote! {
            impl ::std::convert::From<#ident> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> {
                fn from(value: #ident) -> Self {
                    let mut item: Self = ::std::collections::HashMap::new();
                    #(#setters)*
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

#[proc_macro_derive(Dynamodel, attributes(dynamodel))]
pub fn derive_dynamodel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    TargetStruct::from_derive_input(&input)
        .map(TargetStruct::token_stream)
        .unwrap_or_else(|e| e.write_errors().into())
}
