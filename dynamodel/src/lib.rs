pub use dynamodel_derive::Dynamodel;

use aws_sdk_dynamodb::types::AttributeValue;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("`{0}` field is not set")]
    FieldNotSet(String),

    #[error("expect `{0}` type, but got `{1:?}`")]
    AttributeValueUnmatched(String, AttributeValue),

    #[error("{0}")]
    ParseInt(#[from] ParseIntError),

    #[error("{0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}
