mod case;
mod enum_impl;
mod struct_impl;
mod utils;

use case::RenameRule;
use darling::{util::WithOriginal, FromDeriveInput};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

// The main struct we get from parsing the attributes
// Ref: https://github.com/TedDriggs/darling?tab=readme-ov-file#shape-validation
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dynamodel), supports(struct_named, enum_named))]
#[darling(and_then = "TargetStruct::validate")]
struct TargetStruct {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<utils::Variant, WithOriginal<utils::Field, syn::Field>>,
    rename_all: Option<syn::Expr>,
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

    fn token_stream_struct(self) -> TokenStream {
        let ident = self.ident;
        let (imp, ty, whr) = self.generics.split_for_impl();
        let fields = self.data.take_struct().unwrap().fields;

        let rename_rule = self.rename_all.as_ref().map(RenameRule::new);

        let setters = fields.iter().filter_map(|f| {
            if f.parsed.skip_into.is_some_and(|v| v) {
                None
            } else {
                Some(struct_impl::setter::token_stream(&f.parsed, &rename_rule))
            }
        });

        let getters = fields
            .iter()
            .map(|f| struct_impl::getter::token_stream(&f.parsed, &rename_rule));

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
            impl #imp ::std::convert::From<#ident #ty> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> #whr {
                fn from(value: #ident #ty) -> Self {
                    #init_extra

                    let mut item: Self = ::std::collections::HashMap::new();
                    #(#setters)*

                    #set_extra

                    item
                }
            }

            impl #imp ::std::convert::TryFrom<::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>> for #ident #ty #whr {
                type Error = ::dynamodel::ConvertError;

                fn try_from(item: ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#getters,)*
                    })
                }
            }
        }.into()
    }

    fn token_stream_enum(self) -> TokenStream {
        let ident = self.ident;
        let (imp, ty, whr) = self.generics.split_for_impl();
        let variants = self.data.take_enum().unwrap();

        let set_tag: Box<dyn Fn(&syn::Ident) -> proc_macro2::TokenStream> =
            if let Some(ref tag) = self.tag {
                let tag = utils::token_from_str(tag);

                Box::new(move |i: &syn::Ident| {
                    quote! {
                        outer = item;
                        outer.insert(
                            stringify!(#tag).into(),
                            ::aws_sdk_dynamodb::types::AttributeValue::S(stringify!(#i).into()),
                        );
                    }
                })
            } else {
                Box::new(|i: &syn::Ident| {
                    quote! {
                        outer.insert(
                            stringify!(#i).into(),
                            ::aws_sdk_dynamodb::types::AttributeValue::M(item),
                        );
                    }
                })
            };

        let setter_branches = variants
            .iter()
            .map(enum_impl::setter::branch_token(set_tag));

        let getter_branches = variants.iter().map(enum_impl::getter::branch_token);

        let get_variant = if let Some(ref tag) = self.tag {
            let tag = utils::token_from_str(tag);

            quote! {
                let tag = item
                    .get(stringify!(#tag))
                    .ok_or(::dynamodel::ConvertError::FieldNotSet(stringify!(#tag).into()))
                    .and_then(|v| {
                        v.as_s().map_err(|e| {
                            ::dynamodel::ConvertError::AttributeValueUnmatched("S".into(), e.clone())
                        })
                    })
                    .map(|v| v.clone())?;

                match tag.as_str() {
                    #(#getter_branches,)*
                    _ => {},
                }

                Err(::dynamodel::ConvertError::VariantNotFound)
            }
        } else {
            let tag_names = variants.iter().map(|v| {
                let tag = &v.ident;
                quote! { stringify!(#tag) }
            });

            quote! {
                let tags = vec![#(#tag_names,)*];

                for tag in tags {
                    if let Some(::aws_sdk_dynamodb::types::AttributeValue::M(ref item)) = item.get(tag) {
                        match tag {
                            #(#getter_branches,)*
                            _ => {},
                        }
                    }
                }

                Err(::dynamodel::ConvertError::VariantNotFound)
            }
        };

        quote! {
            impl #imp ::std::convert::From<#ident #ty> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> #whr {
                fn from(value: #ident #ty) -> Self {
                    let mut outer: Self = ::std::collections::HashMap::new();
                    let mut item: Self = ::std::collections::HashMap::new();

                    match value {
                        #(#ident::#setter_branches)*
                    }

                    outer
                }
            }

            impl #imp ::std::convert::TryFrom<::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>> for #ident #ty #whr {
                type Error = ::dynamodel::ConvertError;

                fn try_from(item: ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                    #get_variant
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
