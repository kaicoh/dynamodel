use super::{
    case::RenameRule,
    types::{Field, Variant},
    utils::token_from_str,
};

use proc_macro2::TokenStream;
use quote::quote;

pub mod enums;
pub mod structs;
