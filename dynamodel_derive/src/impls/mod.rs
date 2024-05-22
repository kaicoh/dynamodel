use super::{case::RenameRule, utils::*};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub mod enums;
pub mod structs;

fn into_attribute_value(ty: &syn::Type, token: TokenStream) -> TokenStream {
    if is_string(ty) {
        return quote! { S(#token) };
    }

    if is_bool(ty) {
        return quote! { Bool(#token) };
    }

    if is_number(ty) {
        return quote! { N(#token.to_string()) };
    }

    if is_string_vec(ty) {
        return quote! {
            L(#token
              .into_iter()
              .map(::aws_sdk_dynamodb::types::AttributeValue::S)
              .collect())
        };
    }

    if is_bool_vec(ty) {
        return quote! {
            L(#token
              .into_iter()
              .map(::aws_sdk_dynamodb::types::AttributeValue::Bool)
              .collect())
        };
    }

    if is_number_vec(ty) {
        return quote! {
            L(#token
              .into_iter()
              .map(|v| ::aws_sdk_dynamodb::types::AttributeValue::N(v.to_string()))
              .collect())
        };
    }

    if is_any_vec(ty) {
        return quote! {
            L(#token
              .into_iter()
              .map(|v| ::aws_sdk_dynamodb::types::AttributeValue::M(v.into()))
              .collect())
        };
    }

    quote! { M(#token.into()) }
}

fn from_attribute_value(ty: &syn::Type) -> TokenStream {
    if is_string(ty) {
        let err = unmatch_err("S");
        return quote! {
            v.as_s().map(|val| val.clone()).map_err(|e| #err)
        };
    }

    if is_bool(ty) {
        let err = unmatch_err("Bool");
        return quote! {
            v.as_bool().map(|val| *val).map_err(|e| #err)
        };
    }

    if is_number(ty) {
        let err = unmatch_err("N");
        return quote! {
            v.as_n().map_err(|e| #err)
                .and_then(|val| val.parse::<#ty>().map_err(|e| e.into()))
        };
    }

    if is_string_vec(ty) {
        let err = unmatch_err("S");
        return l_wrapper(
            &quote!(String),
            quote! {
                match i.as_s() {
                    Ok(v) => {
                        values.push(v.clone());
                    }
                    Err(e) => {
                        return Err(#err);
                    }
                }
            },
        );
    }

    if is_bool_vec(ty) {
        let err = unmatch_err("Bool");
        return l_wrapper(
            &quote!(bool),
            quote! {
                match i.as_bool() {
                    Ok(v) => {
                        values.push(*v);
                    }
                    Err(e) => {
                        return Err(#err);
                    }
                }
            },
        );
    }

    if is_number_vec(ty) {
        let err = unmatch_err("N");
        if let Some(inner_ty) = inner_type(ty) {
            return l_wrapper(
                inner_ty,
                quote! {
                    match i.as_n().map(|val| val.parse::<#inner_ty>()) {
                        Ok(Ok(v)) => {
                            values.push(v);
                        }
                        Ok(Err(e)) => {
                            return Err(e.into());
                        }
                        Err(e) => {
                            return Err(#err);
                        }
                    }
                },
            );
        } else {
            unreachable!("expect a vector of number, got {ty:#?}");
        }
    }

    if is_any_vec(ty) {
        let err = unmatch_err("M");
        if let Some(inner_ty) = inner_type(ty) {
            return l_wrapper(
                inner_ty,
                quote! {
                    match i.as_m().map(|val| #inner_ty::try_from(val.clone())) {
                        Ok(Ok(v)) => {
                            values.push(v);
                        }
                        Ok(Err(e)) => {
                            return Err(e);
                        }
                        Err(e) => {
                            return Err(#err);
                        }
                    }
                },
            );
        } else {
            unreachable!("expect a vector of model implementing TryFrom<HashMap<String, AttributeValue>>, got {ty:#?}");
        }
    }

    let err = unmatch_err("M");
    quote! {
        v.as_m().map_err(|e| #err)
            .and_then(|val| #ty::try_from(val.clone()))
    }
}

fn not_set_err(ident: &Option<syn::Ident>) -> TokenStream {
    quote! {
        ::dynamodel::ConvertError::FieldNotSet(stringify!(#ident).into())
    }
}

fn unmatch_err(ty: &str) -> TokenStream {
    quote! {
        ::dynamodel::ConvertError::AttributeValueUnmatched(#ty.into(), e.clone())
    }
}

fn l_wrapper(ty: &impl ToTokens, token: TokenStream) -> TokenStream {
    let l_err = unmatch_err("L");
    quote! {
        v.as_l().map_err(|e| #l_err).and_then(|l| {
            let mut values: Vec<#ty> = vec![];
            for i in l.iter() { #token }
            Ok(values)
        })
    }
}

fn is_string(ty: &syn::Type) -> bool {
    type_is("String", ty)
}

fn is_number(ty: &syn::Type) -> bool {
    is_u8(ty)
        || is_u16(ty)
        || is_u32(ty)
        || is_u64(ty)
        || is_u128(ty)
        || is_usize(ty)
        || is_i8(ty)
        || is_i16(ty)
        || is_i32(ty)
        || is_i64(ty)
        || is_i128(ty)
        || is_isize(ty)
        || is_f32(ty)
        || is_f64(ty)
}

fn is_any_vec(ty: &syn::Type) -> bool {
    type_is("Vec", ty)
}

fn type_is(literal: &str, ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty
    {
        if segments[0].ident == literal {
            return true;
        }
    }
    false
}

fn inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty
    {
        if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            ref args,
            ..
        }) = segments[0].arguments
        {
            let ty = match args[0] {
                syn::GenericArgument::Type(ref t) => Some(t),
                _ => None,
            };
            return ty;
        }
    }
    None
}

fn inner_type_of<'a>(literal: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if !type_is(literal, ty) {
        return None;
    }
    inner_type(ty)
}

macro_rules! type_is_fn {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                fn [<is_$ty>](ty: &syn::Type) -> bool {
                    type_is(stringify!($ty), ty)
                }
            }
         )*
    }
}

type_is_fn!(bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! type_is_vec_fn {
    ($($ident:ident),*) => {
        $(
            paste::item! {
                pub fn [<$ident _vec>](ty: &syn::Type) -> bool {
                    if !type_is("Vec", ty) {
                        return false;
                    }

                    match inner_type(ty) {
                        Some(ty) => $ident(ty),
                        None => false,
                    }
                }
            }
         )*
    }
}

type_is_vec_fn!(is_string, is_bool, is_number);
