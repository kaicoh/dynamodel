use dynamodel::{Dynamodel, ConvertError};
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

type H = HashMap<String, AttributeValue>;

#[derive(Dynamodel)]
struct ExampleStruct<T>
where
    T: Into<H> + TryFrom<H, Error = ConvertError>,
{
    attr: T,
}

#[derive(Dynamodel)]
enum ExampleEnum<T>
where
    T: Into<H> + TryFrom<H, Error = ConvertError>,
{
    A { attr: T },
    B { attr: String },
}

fn main() {}
