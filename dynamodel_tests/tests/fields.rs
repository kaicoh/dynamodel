use aws_sdk_dynamodb::types::AttributeValue;
use dynamodel::{ConvertError, Dynamodel};
use std::collections::HashMap;

#[macro_use]
mod macros;

mod attribute;
mod bool;
mod inner_struct;
mod number;
mod string;
