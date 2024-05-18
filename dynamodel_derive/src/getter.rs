use super::utils::*;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn token_stream(field: &Field) -> TokenStream {
    let field_name = &field.ident;
    let ty = &field.ty;

    let field_not_set = not_set_err(field_name);

    if let Some(ref converter) = field.try_from {
        return quote! {
            #field_name: item
                .get(stringify!(#field_name))
                .ok_or(#field_not_set)
                .and_then(#converter)?
        };
    }

    match inner_type_of("Option", ty) {
        Some(ty) => {
            let into_value = from_attribute_value(ty);

            quote! {
                #field_name: item
                    .get(stringify!(#field_name))
                    .map(|v| { #into_value })
                    .transpose()?
            }
        }
        None => {
            let into_value = from_attribute_value(ty);

            quote! {
                #field_name: item
                    .get(stringify!(#field_name))
                    .ok_or(#field_not_set)
                    .and_then(|v| { #into_value })?
            }
        }
    }
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
