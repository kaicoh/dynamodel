mod case;
mod types;
mod utils;

use case::RenameRule;
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use types::{NamedField, NamedVariant};

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
    data: darling::ast::Data<types::Variant, types::Field>,
    rename_all: Option<syn::Lit>,
    extra: Option<darling::Result<syn::Path>>,
    tag: Option<String>,
}

impl TargetStruct {
    fn validate(self) -> darling::Result<Self> {
        match &self.data {
            darling::ast::Data::Struct(fields) => {
                for field in fields.fields.iter() {
                    field.validate();
                }
            }
            darling::ast::Data::Enum(variants) => {
                for variant in variants {
                    variant.validate();
                }
            }
        }

        Ok(self)
    }

    fn extra(&self) -> Option<syn::Path> {
        match self.extra.clone().transpose() {
            Ok(v) => v,
            Err(err) => {
                abort! {
                    err.span(), "Invalid attribute #[dynamodel(extra = ...)]";
                    note = "Invalid argument for `extra` attribute. Only paths are allowed.";
                    help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`";
                }
            }
        }
    }

    fn rename_rule(&self) -> RenameRule {
        self.rename_all
            .as_ref()
            .map(RenameRule::from_lit)
            .unwrap_or_default()
    }

    fn impl_traits(&self) -> impl Fn(TokenStream2, TokenStream2) -> TokenStream + '_ {
        let ident = self.ident.clone();
        let (imp, ty, whr) = self.generics.split_for_impl();

        move |from_impl, try_from_impl| {
            quote! {
                impl #imp ::std::convert::From<#ident #ty> for ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue> #whr {
                    fn from(value: #ident #ty) -> Self {
                        #from_impl
                    }
                }

                impl #imp ::std::convert::TryFrom<::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>> for #ident #ty #whr {
                    type Error = ::dynamodel::ConvertError;

                    fn try_from(item: ::std::collections::HashMap<String, ::aws_sdk_dynamodb::types::AttributeValue>) -> Result<Self, Self::Error> {
                        #try_from_impl
                    }
                }
            }.into()
        }
    }

    fn struct_token(self) -> TokenStream {
        let ident = &self.ident;
        let impl_traits = self.impl_traits();
        let rename_rule = self.rename_rule();

        let init_hashmap = match self.extra() {
            Some(path) => quote! { #path(&value); },
            None => quote! { ::std::collections::HashMap::new(); },
        };

        let set_tag = if let Some(tag) = self.tag.as_ref() {
            quote! {
                item.insert(
                    #tag.into(),
                    ::aws_sdk_dynamodb::types::AttributeValue::S(stringify!(#ident).into()),
                );
            }
        } else {
            quote!()
        };

        let fields: Vec<NamedField> = self
            .data
            .clone()
            .take_struct()
            .unwrap()
            .fields
            .into_iter()
            .map(|f| f.into_named(&rename_rule))
            .collect();

        let set_key_values = fields.iter().filter_map(|f| {
            if f.skip_into() {
                None
            } else {
                Some(f.set_key_value_pair_token(|v| quote!(value.#v)))
            }
        });

        let set_named_fields = fields.iter().map(NamedField::set_named_field_token);

        let from_impl = quote! {
            let mut item: Self = #init_hashmap
            #(#set_key_values)*
            #set_tag
            item
        };

        let try_from_impl = quote! {
            Ok(Self { #(#set_named_fields,)* })
        };

        impl_traits(from_impl, try_from_impl)
    }

    fn enum_token(self) -> TokenStream {
        let ident = &self.ident;
        let impl_traits = self.impl_traits();
        let rename_rule = self.rename_rule();

        let variants: Vec<NamedVariant> = self
            .data
            .clone()
            .take_enum()
            .unwrap()
            .into_iter()
            .map(|v| v.into_named(&rename_rule))
            .collect();

        let set_key_value_branch = variants.iter().map(NamedVariant::set_key_value);
        let get_values = variants.iter().map(NamedVariant::get_value_token);

        let from_impl = quote! {
            match value {
                #(#ident::#set_key_value_branch)*
            }
        };

        let try_from_impl = quote! {
            #(#get_values)*
            Err(::dynamodel::ConvertError::VariantNotFound)
        };

        impl_traits(from_impl, try_from_impl)
    }

    fn enum_token_tagged(self) -> TokenStream {
        let ident = &self.ident;
        let impl_traits = self.impl_traits();
        let rename_rule = self.rename_rule();
        let tag = self.tag.clone().unwrap();
        let tag_str = tag.as_str();

        let variants: Vec<NamedVariant> = self
            .data
            .clone()
            .take_enum()
            .unwrap()
            .into_iter()
            .map(|v| v.into_named(&rename_rule))
            .collect();

        let set_key_value_branch = variants.iter().map(|v| v.set_tagged_key_value(&tag));
        let get_values = variants.iter().map(NamedVariant::get_value_token_tagged);

        let from_impl = quote! {
            match value {
                #(#ident::#set_key_value_branch)*
            }
        };

        let try_from_impl = quote! {
            let tag = item
                .get(#tag_str)
                .ok_or(::dynamodel::ConvertError::FieldNotSet(stringify!(#tag).into()))
                .and_then(|v| {
                    v.as_s().map_err(|e| {
                        ::dynamodel::ConvertError::AttributeValueUnmatched("S".into(), e.clone())
                    })
                })
                .map(|v| v.clone())?;

            match tag.as_str() {
                #(#get_values,)*
                _ => {},
            }

            Err(::dynamodel::ConvertError::VariantNotFound)
        };

        impl_traits(from_impl, try_from_impl)
    }

    fn token_stream(self) -> TokenStream {
        match self.data {
            darling::ast::Data::Struct(_) => self.struct_token(),
            darling::ast::Data::Enum(_) => {
                if self.tag.is_some() {
                    self.enum_token_tagged()
                } else {
                    self.enum_token()
                }
            }
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
