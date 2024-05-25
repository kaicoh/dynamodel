use super::*;

pub fn getter_token(field: &Field, rename_rule: &RenameRule) -> TokenStream {
    let field_name = &field.ident;
    let ty = &field.ty;
    let field_not_set = not_set_err(field_name);
    let hash_key = field.renamed(rename_rule);

    if let Some(ref converter) = field.try_from_item {
        return quote! {
            #field_name: #converter(&item)?
        };
    }

    let get_value = quote! { item.get(stringify!(#hash_key)) };

    if let Some(ref converter) = field.try_from {
        return quote! {
            #field_name: #get_value
                .ok_or(#field_not_set)
                .and_then(#converter)?
        };
    }

    match inner_type_of("Option", ty) {
        Some(ty) => {
            let into_value = from_attribute_value(ty);

            quote! {
                #field_name: #get_value
                    .map(|v| { #into_value })
                    .transpose()?
            }
        }
        None => {
            let into_value = from_attribute_value(ty);

            quote! {
                #field_name: #get_value
                    .ok_or(#field_not_set)
                    .and_then(|v| { #into_value })?
            }
        }
    }
}

pub fn setter_token(field: &Field, rename_rule: &RenameRule) -> TokenStream {
    let field_name = &field.ident;
    let ty = &field.ty;
    let hash_key_token = field.renamed(rename_rule);
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
                        ::aws_sdk_dynamodb::types::AttributeValue::#variant,
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
                    ::aws_sdk_dynamodb::types::AttributeValue::#variant,
                );
            }
        }
    }
}

fn attribute_value_variant(ty: &syn::Type) -> TokenStream {
    into_attribute_value(ty, quote!(v))
}
