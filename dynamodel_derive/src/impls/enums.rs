use super::*;

pub fn into_hashmap(
    ident: &syn::Ident,
    tag: &Option<String>,
    variants: &[Variant],
    rule: &RenameRule,
) -> TokenStream {
    let (init, return_value) = if tag.is_some() {
        (quote!(), quote!(item))
    } else {
        (
            quote! { let mut outer: Self = ::std::collections::HashMap::new(); },
            quote! { outer },
        )
    };

    let branches = variants.iter().map(|v| v.setters(tag, rule));

    quote! {
        #init
        let mut item: Self = ::std::collections::HashMap::new();
        match value {
            #(#ident::#branches)*
        }
        #return_value
    }
}

pub fn try_from_hashmap(
    tag: &Option<String>,
    variants: &[Variant],
    rule: &RenameRule,
) -> TokenStream {
    if let Some(ref tag) = tag {
        internally_tagged(tag, variants, rule)
    } else {
        externally_tagged(variants, rule)
    }
}

fn internally_tagged(tag: &String, variants: &[Variant], rule: &RenameRule) -> TokenStream {
    let tag_token = token_from_str(tag);
    let branches = variants.iter().map(|v| v.getters(rule));

    quote! {
        let tag = item
            .get(stringify!(#tag_token))
            .ok_or(::dynamodel::ConvertError::FieldNotSet(stringify!(#tag).into()))
            .and_then(|v| {
                v.as_s().map_err(|e| {
                    ::dynamodel::ConvertError::AttributeValueUnmatched("S".into(), e.clone())
                })
            })
            .map(|v| v.clone())?;

        match tag.as_str() {
            #(#branches,)*
            _ => {},
        }

        Err(::dynamodel::ConvertError::VariantNotFound)
    }
}

fn externally_tagged(variants: &[Variant], rule: &RenameRule) -> TokenStream {
    let (newtypes, named_fields): (Vec<&Variant>, Vec<&Variant>) =
        variants.iter().partition(|&v| v.is_newtype());

    let newtypes_token = externally_tagged_newtype(&newtypes, rule);
    let named_fields_token = externally_tagged_named_fields(&named_fields, rule);

    quote! {
        #newtypes_token
        #named_fields_token
        Err(::dynamodel::ConvertError::VariantNotFound)
    }
}

fn externally_tagged_named_fields(variants: &[&Variant], rule: &RenameRule) -> TokenStream {
    if variants.is_empty() {
        return quote!();
    }

    let names = variants.iter().map(|&v| {
        let renamed = v.renamed(rule);
        quote!(stringify!(#renamed))
    });
    let branches = variants.iter().map(|&v| v.getters(rule));

    quote! {
        for variant in [#(#names,)*] {
            match item.get(variant) {
                Some(::aws_sdk_dynamodb::types::AttributeValue::M(ref item)) => {
                    match variant {
                        #(#branches,)*
                        _ => {},
                    }
                }
                Some(err) => {
                    return Err(::dynamodel::ConvertError::AttributeValueUnmatched("M".into(), err.clone()));
                }
                None => {}
            }
        }
    }
}

fn externally_tagged_newtype(variants: &[&Variant], rule: &RenameRule) -> TokenStream {
    if variants.is_empty() {
        return quote!();
    }

    let branches = variants.iter().map(|&v| v.getters(rule));

    quote! {
        #(#branches)*
    }
}
