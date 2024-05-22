use super::*;

pub fn getter_branch(variant: &Variant) -> TokenStream {
    let name = &variant.ident;
    let fields = &variant.fields.fields;

    let field_getters = fields.iter().map(|f| {
        let field_name = &f.ident;
        let ty = &f.ty;
        let field_not_set = not_set_err(field_name);

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
    });

    quote! {
        stringify!(#name) => {
            return Ok(Self::#name { #(#field_getters,)* });
        }
    }
}

pub fn setter_branch<T>(set_tag: T) -> Box<dyn Fn(&Variant) -> TokenStream>
where
    T: Fn(&syn::Ident) -> TokenStream + 'static,
{
    Box::new(move |variant: &Variant| {
        let name = &variant.ident;
        let fields = &variant.fields.fields;

        let field_names = fields.iter().map(|f| &f.ident);

        let field_setters = fields.iter().map(|f| {
            let field_name = &f.ident;
            let ty = &f.ty;

            match inner_type_of("Option", ty) {
                Some(ty) => {
                    let attribute_value = attribute_value_variant(field_name, ty);

                    quote! {
                        if let Some(#field_name) = #field_name {
                            item.insert(
                                stringify!(#field_name).into(),
                                ::aws_sdk_dynamodb::types::AttributeValue::#attribute_value,
                            );
                        }
                    }
                }
                None => {
                    let attribute_value = attribute_value_variant(field_name, ty);

                    quote! {
                        item.insert(
                            stringify!(#field_name).into(),
                            ::aws_sdk_dynamodb::types::AttributeValue::#attribute_value,
                        );
                    }
                }
            }
        });

        let set_variant_name_to_outer = set_tag(name);

        quote! {
            #name { #(#field_names,)* } => {
                #(#field_setters)*
                #set_variant_name_to_outer
            }
        }
    })
}

fn attribute_value_variant(ident: &Option<syn::Ident>, ty: &syn::Type) -> TokenStream {
    into_attribute_value(ty, quote!(#ident))
}
