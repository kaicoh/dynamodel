use super::utils::*;

use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream(field: &Field) -> TokenStream {
    let field_name = &field.ident;
    let ty = &field.ty;

    if let Some(ref converter) = field.into {
        return quote! {
            let v = value.#field_name;
            item.insert(
                stringify!(#field_name).into(),
                #converter(v),
            );
        };
    }

    match inner_type_of("Option", ty) {
        Some(ty) => {
            let attr = attribute_value_type(ty);

            quote! {
                if let Some(v) = value.#field_name {
                    item.insert(
                        stringify!(#field_name).into(),
                        aws_sdk_dynamodb::types::AttributeValue::#attr,
                    );
                }
            }
        }
        None => {
            let attr = attribute_value_type(ty);

            quote! {
                let v = value.#field_name;
                item.insert(
                    stringify!(#field_name).into(),
                    aws_sdk_dynamodb::types::AttributeValue::#attr,
                );
            }
        }
    }
}

fn attribute_value_type(ty: &syn::Type) -> TokenStream {
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
