use super::*;
use proc_macro_error::abort;

pub fn into_hashmap(
    ident: &syn::Ident,
    tag: &Option<String>,
    extra: &Option<darling::Result<syn::Path>>,
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

fn init_hashmap(extra: &Option<darling::Result<syn::Path>>) -> TokenStream {
    let init = match extra.as_ref() {
        Some(Ok(path)) => {
            quote! { #path(&value); }
        }
        Some(Err(err)) => {
            abort! {
                err.span(), "Invalid attribute #[dynamodel(extra = ...)]";
                note = "Invalid argument for `extra` attribute. Only paths are allowed.";
                help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`";
            }
        }
        None => {
            quote! { ::std::collections::HashMap::new(); }
        }
    };

    quote! { let mut item: Self = #init }
}

pub fn try_from_hashmap(fields: &[Field], rule: &RenameRule) -> TokenStream {
    let getters = fields.iter().map(|f| f.named_getter(rule));

    quote! {
        Ok(Self { #(#getters,)* })
    }
}
