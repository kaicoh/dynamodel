pub fn is_string(ty: &syn::Type) -> bool {
    type_is("String", ty)
}

pub fn is_number(ty: &syn::Type) -> bool {
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

pub fn is_any_vec(ty: &syn::Type) -> bool {
    type_is("Vec", ty)
}

pub fn type_is(literal: &str, ty: &syn::Type) -> bool {
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

pub fn inner_type(ty: &syn::Type) -> Option<&syn::Type> {
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

pub fn inner_type_of<'a>(literal: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if !type_is(literal, ty) {
        return None;
    }
    inner_type(ty)
}

macro_rules! type_is_fn {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                pub fn [<is_$ty>](ty: &syn::Type) -> bool {
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
