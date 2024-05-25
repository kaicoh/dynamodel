use super::*;

pub fn try_from_hashmap(
    tag: &Option<String>,
    rule: &RenameRule,
    variants: &[Variant],
) -> TokenStream {
    let getter_branches = variants.iter().map(getter_branch(rule));

    if let Some(ref tag) = tag {
        let tag = token_from_str(tag);

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
            let renamed = v.renamed(rule);
            quote!(stringify!(#renamed))
        });

        quote! {
            let tags = vec![#(#tag_names,)*];

            for tag in tags {
                match item.get(tag) {
                    Some(::aws_sdk_dynamodb::types::AttributeValue::M(ref item)) => {
                        match tag {
                            #(#getter_branches,)*
                            _ => {},
                        }
                    }
                    Some(err) => {
                        return Err(::dynamodel::ConvertError::AttributeValueUnmatched("M".into(), err.clone()));
                    }
                    None => {}
                }
            }

            Err(::dynamodel::ConvertError::VariantNotFound)
        }
    }
}

fn getter_branch(rule: &RenameRule) -> Box<dyn Fn(&Variant) -> TokenStream> {
    let rule = *rule;

    Box::new(move |variant: &Variant| {
        let name = &variant.ident;
        let fields = &variant.fields.fields;
        let field_rename_rule = variant.rename_rule_for_fields();

        let field_getters = fields.iter().map(|f| {
            let field_name = &f.ident;
            let ty = &f.ty;
            let field_not_set = not_set_err(field_name);
            let hash_key = f.renamed(&field_rename_rule);

            let get_value = quote! { item.get(stringify!(#hash_key)) };

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
        });

        let renamed = variant.renamed(&rule);

        quote! {
            stringify!(#renamed) => {
                return Ok(Self::#name { #(#field_getters,)* });
            }
        }
    })
}

pub fn into_hashmap(
    ident: &syn::Ident,
    tag: &Option<String>,
    rule: &RenameRule,
    variants: &[Variant],
) -> TokenStream {
    let set_outer = if tag.is_some() {
        quote!()
    } else {
        quote! { let mut outer: Self = ::std::collections::HashMap::new(); }
    };

    let return_value = if tag.is_some() {
        quote! { item }
    } else {
        quote! { outer }
    };

    let setter_branches = variants.iter().map(setter_branch(tag, rule));

    quote! {
        #set_outer
        let mut item: Self = ::std::collections::HashMap::new();

        match value {
            #(#ident::#setter_branches)*
        }

        #return_value
    }
}

pub fn setter_branch(
    tag: &Option<String>,
    rule: &RenameRule,
) -> Box<dyn Fn(&Variant) -> TokenStream> {
    let tag = tag.clone();
    let rule = *rule;

    Box::new(move |variant: &Variant| {
        let name = &variant.ident;
        let fields = &variant.fields.fields;
        let field_rename_rule = variant.rename_rule_for_fields();

        let field_names = fields.iter().map(|f| &f.ident);

        let field_setters = fields.iter().map(|f| {
            let field_name = &f.ident;
            let ty = &f.ty;
            let hash_key_token = f.renamed(&field_rename_rule);
            let hash_key = quote! { stringify!(#hash_key_token).into() };

            match inner_type_of("Option", ty) {
                Some(ty) => {
                    let attribute_value = attribute_value_variant(field_name, ty);

                    quote! {
                        if let Some(#field_name) = #field_name {
                            item.insert(
                                #hash_key,
                                ::aws_sdk_dynamodb::types::AttributeValue::#attribute_value,
                            );
                        }
                    }
                }
                None => {
                    let attribute_value = attribute_value_variant(field_name, ty);

                    quote! {
                        item.insert(
                            #hash_key,
                            ::aws_sdk_dynamodb::types::AttributeValue::#attribute_value,
                        );
                    }
                }
            }
        });

        let renamed_variant = variant.renamed(&rule);
        let renamed = quote! { stringify!(#renamed_variant).into() };

        let set_variant = if let Some(ref tag) = tag {
            let tag = token_from_str(tag);

            quote! {
                item.insert(
                    stringify!(#tag).into(),
                    ::aws_sdk_dynamodb::types::AttributeValue::S(#renamed),
                );
            }
        } else {
            quote! {
                outer.insert(
                    #renamed,
                    ::aws_sdk_dynamodb::types::AttributeValue::M(item),
                );
            }
        };

        quote! {
            #name { #(#field_names,)* } => {
                #(#field_setters)*
                #set_variant
            }
        }
    })
}

fn attribute_value_variant(ident: &Option<syn::Ident>, ty: &syn::Type) -> TokenStream {
    into_attribute_value(ty, quote!(#ident))
}
