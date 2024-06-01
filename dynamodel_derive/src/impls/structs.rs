use super::*;
use proc_macro_error::abort;

pub fn into_hashmap(
    ident: &syn::Ident,
    tag: &Option<String>,
    extra: &Option<darling::Result<syn::Path>>,
    fields: &[Field],
    rule: &RenameRule,
) -> TokenStream {
    let token = wrap_extra(extra, build_hashmap(fields, rule));

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
        #token
        #set_tag
        item
    }
}

fn build_hashmap(fields: &[Field], rule: &RenameRule) -> TokenStream {
    let setters = fields.iter().filter_map(|f| into_setter_token(f, rule));

    quote! {
        let mut item: Self = ::std::collections::HashMap::new();
        #(#setters)*
    }
}

fn into_setter_token(f: &Field, rule: &RenameRule) -> Option<TokenStream> {
    if f.skip_into.is_some_and(|v| v) {
        None
    } else {
        Some(f.named_setter(rule, |v| quote! { value.#v }))
    }
}

pub fn try_from_hashmap(fields: &[Field], rule: &RenameRule) -> TokenStream {
    let getters = fields.iter().map(|f| f.named_getter(rule));

    quote! {
        Ok(Self { #(#getters,)* })
    }
}

fn wrap_extra(extra: &Option<darling::Result<syn::Path>>, inner: TokenStream) -> TokenStream {
    let (init, set) = match extra.as_ref() {
        Some(Ok(path)) => {
            let init = quote! {
                let extra_item: Self = #path(&value);
            };
            let set = quote! { item.extend(extra_item); };
            (init, set)
        }
        Some(Err(err)) => {
            abort! {
                err.span(), "Invalid attribute #[dynamodel(extra = ...)]";
                note = "Invalid argument for `extra` attribute. Only paths are allowed.";
                help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`";
            }
        }
        None => (quote!(), quote!()),
    };

    quote! {
        #init
        #inner
        #set
    }
}
