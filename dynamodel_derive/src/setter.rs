use super::case::RenameRule;
use super::utils::*;

use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream(field: &Field, rename_rule: &Option<RenameRule>) -> TokenStream {
    let field_name = &field.ident;
    let ty = &field.ty;
    let hash_key_token = hash_key_name(field, rename_rule);
    let hash_key = quote! { stringify!(#hash_key_token).into() };

    if let Some(ref converter) = field.into {
        return quote! {
            let v = value.#field_name;
            item.insert(#hash_key, #converter(v));
        };
    }

    match inner_type_of("Option", ty) {
        Some(ty) => {
            let variant = attribute_value_variant(ty);

            quote! {
                if let Some(v) = value.#field_name {
                    item.insert(
                        #hash_key,
                        aws_sdk_dynamodb::types::AttributeValue::#variant,
                    );
                }
            }
        }
        None => {
            let variant = attribute_value_variant(ty);

            quote! {
                let v = value.#field_name;
                item.insert(
                    #hash_key,
                    aws_sdk_dynamodb::types::AttributeValue::#variant,
                );
            }
        }
    }
}

fn attribute_value_variant(ty: &syn::Type) -> TokenStream {
    if is_string(ty) {
        // type is "String"
        return quote! { S(v) };
    }

    if is_bool(ty) {
        // type is "bool"
        return quote! { Bool(v) };
    }

    if is_number(ty) {
        // type is one of the u8, u16, u32, u64, u128, usize,
        // i8, i16, i32, i64, i128, isize, f32 and f64
        return quote! { N(v.to_string()) };
    }

    if is_string_vec(ty) {
        // type is Vec<String>
        return quote! {
            L(v.into_iter()
              .map(aws_sdk_dynamodb::types::AttributeValue::S)
              .collect())
        };
    }

    if is_bool_vec(ty) {
        // type is Vec<bool>
        return quote! {
            L(v.into_iter()
              .map(aws_sdk_dynamodb::types::AttributeValue::Bool)
              .collect())
        };
    }

    if is_number_vec(ty) {
        // type is Vec<any number type>
        return quote! {
            L(v.into_iter()
              .map(|v| aws_sdk_dynamodb::types::AttributeValue::N(v.to_string()))
              .collect())
        };
    }

    if is_any_vec(ty) {
        return quote! {
            L(v.into_iter()
              .map(|v| aws_sdk_dynamodb::types::AttributeValue::M(v.into()))
              .collect())
        };
    }

    quote! { M(v.into()) }
}
