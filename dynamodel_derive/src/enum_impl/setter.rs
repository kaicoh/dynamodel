use super::utils::*;

use proc_macro2::TokenStream;
use quote::quote;

pub fn branch_token<T>(set_tag: T) -> Box<dyn Fn(&Variant) -> TokenStream>
where
    T: Fn(&syn::Ident) -> TokenStream + 'static,
{
    Box::new(move |variant: &Variant| {
        let name = &variant.ident;
        let fields = &variant.fields.fields;

        let field_names = fields.iter().map(|f| &f.ident);

        let field_setters = fields.iter().map(|f| {
            let field_name = &f.ident;
            let attribute_value = attribute_value_variant(f);

            quote! {
                item.insert(
                    stringify!(#field_name).into(),
                    ::aws_sdk_dynamodb::types::AttributeValue::#attribute_value,
                );
            }
        });

        let set_variant_name_to_outer = set_tag(name);

        quote! {
            #name { #(#field_names,)* } => {
                #(#field_setters)*
                #set_variant_name_to_outer
            }
        }
    })
}

fn attribute_value_variant(field: &Field) -> TokenStream {
    let name = &field.ident;
    let ty = &field.ty;

    if is_string(ty) {
        return quote! { S(#name) };
    }

    if is_bool(ty) {
        return quote! { Bool(#name) };
    }

    if is_number(ty) {
        return quote! { N(#name.to_string()) };
    }

    if is_string_vec(ty) {
        return quote! {
            L(#name
              .into_iter()
              .map(::aws_sdk_dynamodb::types::AttributeValue::S)
              .collect())
        };
    }

    if is_bool_vec(ty) {
        return quote! {
            L(#name
              .into_iter()
              .map(::aws_sdk_dynamodb::types::AttributeValue::Bool)
              .collect())
        };
    }

    if is_number_vec(ty) {
        return quote! {
            L(#name
              .into_iter()
              .map(|v| ::aws_sdk_dynamodb::types::AttributeValue::N(v.to_string()))
              .collect())
        };
    }

    if is_any_vec(ty) {
        return quote! {
            L(#name
              .into_iter()
              .map(|v| ::aws_sdk_dynamodb::types::AttributeValue::M(v.into()))
              .collect())
        };
    }

    quote! { M(#name.into()) }
}
