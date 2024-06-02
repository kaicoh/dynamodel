use super::*;

pub fn into_hashmap(
    ident: &syn::Ident,
    tag: &Option<String>,
    extra: Option<syn::Path>,
    fields: &[Field],
    rule: &RenameRule,
) -> TokenStream {
    let init = init_hashmap(extra);
    let set_values = set_hashmap_values(fields, rule);

    let set_tag = if let Some(ref tag) = tag {
        quote! {
            item.insert(
                #tag.into(),
                ::aws_sdk_dynamodb::types::AttributeValue::S(stringify!(#ident).into()),
            );
        }
    } else {
        quote!()
    };

    quote! {
        #init
        #set_values
        #set_tag
        item
    }
}

fn set_hashmap_values(fields: &[Field], rule: &RenameRule) -> TokenStream {
    let setters = fields.iter().filter_map(|f| {
        if f.skip_into.is_some_and(|v| v) {
            None
        } else {
            Some(f.named_setter(rule, |v| quote! { value.#v }))
        }
    });

    quote! { #(#setters)* }
}

fn init_hashmap(extra: Option<syn::Path>) -> TokenStream {
    let init = match extra {
        Some(path) => quote! { #path(&value); },
        None => quote! { ::std::collections::HashMap::new(); },
    };

    quote! { let mut item: Self = #init }
}

pub fn try_from_hashmap(fields: &[Field], rule: &RenameRule) -> TokenStream {
    let fields_token = fields.iter().map(|f| f.set_named_field_token(rule));

    quote! {
        Ok(Self { #(#fields_token,)* })
    }
}
