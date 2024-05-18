use dynamodel::Dynamodel;
use aws_sdk_dynamodb::{types::AttributeValue, primitives::Blob};

#[derive(Dynamodel)]
struct InvalidAttributeDefinition {
    #[dynamodel(into("from_vec_to_blob"))]
    bytes: Vec<u8>,
}

fn main() {}

fn from_vec_to_blob(value: Vec<u8>) -> AttributeValue {
    AttributeValue::B(Blob::new(value))
}
