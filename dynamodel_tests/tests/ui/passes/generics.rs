use dynamodel::{Dynamodel, ConvertError};
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

type H = HashMap<String, AttributeValue>;

#[derive(Dynamodel)]
struct Model<T>
where
    T: Into<H> + TryFrom<H, Error = ConvertError>,
{
    attr: T,
}

fn main() {}
