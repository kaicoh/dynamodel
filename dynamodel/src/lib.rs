//! # dynamodel
//! This library provide a derive macro to implement conversion between your struct and
//! [HashMap](std::collections::HashMap)<[String], [AttributeValue]>.
//!
//! ## Usage
//!
//! ```rust
//! use dynamodel::Dynamodel;
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::types::AttributeValue;
//!
//! // Using `Dynamodel` macro, you can implement both
//! // `From<your struct> for HashMap<String, AttributeValue>` and
//! // `TryFrom<HashMap<String, AttributeValue>> for your struct` traits.
//! #[derive(Dynamodel, Debug, Clone, PartialEq)]
//! struct Person {
//!     first_name: String,
//!     last_name: String,
//!     age: u8,
//! }
//!
//! let person = Person {
//!     first_name: "Kanji".into(),
//!     last_name: "Tanaka".into(),
//!     age: 23,
//! };
//!
//! let item: HashMap<String, AttributeValue> = [
//!     ("first_name".to_string(), AttributeValue::S("Kanji".into())),
//!     ("last_name".to_string(), AttributeValue::S("Tanaka".into())),
//!     ("age".to_string(), AttributeValue::N("23".into()))
//! ].into();
//!
//! // Convert from Person into HashMap<String, AttributeValue>.
//! let converted: HashMap<String, AttributeValue> = person.clone().into();
//! assert_eq!(converted, item);
//!
//! // Convert from HashMap<String, AttributeValue> into Person.
//! // This conversion uses std::convert::TryFrom trait, so this returns a Result.
//! let converted: Person = item.try_into().unwrap();
//! assert_eq!(converted, person);
//! ```
//!
//! ## Implicit conversion
//!
//! This macro converts some types implicitly so you don't have to add any code. These types are
//! followings.
//!
//! | Type | AttributeValue variant | memo |
//! |---|---|---|
//! | `String` | `AttributeValue::S` |  |
//! | `u8, u16, u32, u64, u128, usize`<br>`i8, i16, i32, i64, i128, isize`<br>`f32, f64` | `AttributeValue::N` |  |
//! | `bool` | `AttributeValue::Bool` |  |
//! | Any struct that implements `Dynamodel` macro | `AttributeValue::M` |  |
//! | `Vec<inner type>` | `AttributeValue::L` | The inner type must be one of the implicit conversion types. |
//! | `Option<inner type>` | Depends on the inner type | The inner type must be one of the implicit conversion types. |
//!
//! ## Explicit conversion
//!
//! Using field attribute, you can implement original conversion methods like this.
//!
//! ```rust
//! use dynamodel::{Dynamodel, ConvertError};
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::{types::AttributeValue, primitives::Blob};
//!
//! // Vec<u8> is converted to AttributeValue::L by default,
//! // but this case, the `data` field is converted to AttributeValue::B.
//! #[derive(Dynamodel)]
//! struct BinaryData {
//!     #[dynamodel(into = "to_blob", try_from = "from_blob")]
//!     data: Vec<u8>
//! }
//!
//! fn to_blob(value: Vec<u8>) -> AttributeValue {
//!     AttributeValue::B(Blob::new(value))
//! }
//!
//! fn from_blob(value: &AttributeValue) -> Result<Vec<u8>, ConvertError> {
//!     value.as_b()
//!         .map(|b| b.clone().into_inner())
//!         .map_err(|err| ConvertError::AttributeValueUnmatched("B".to_string(), err.clone()))
//! }
//! ```
//!
//! The function definition must satisfy these conditions.
//!
//! | Conversion | Argument | Return |
//! |---|---|---|
//! | `field type => AttributeValue` | `field type` | [`AttributeValue`] |
//! | `AttributeValue => field type` | `&`[`AttributeValue`] | [`Result`]`<field type,`[`ConvertError`]`>` |

/// Derive macro to implement conversion between your struct and
/// [HashMap](std::collections::HashMap)<[String], [AttributeValue]>.
pub use dynamodel_derive::Dynamodel;

use aws_sdk_dynamodb::types::AttributeValue;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/// A conversion error from [AttributeValue] to your struct field.
#[derive(Debug, Error)]
pub enum ConvertError {
    /// Occurs when the [HashMap](std::collections::HashMap)<[String], [AttributeValue]> has no key value pair for the field.
    #[error("`{0}` field is not set")]
    FieldNotSet(String),

    /// Occurs when the [HashMap](std::collections::HashMap)<[String], [AttributeValue]> has a key value pair for the field but its variant is match.
    #[error("expect `{0}` type, but got `{1:?}`")]
    AttributeValueUnmatched(String, AttributeValue),

    /// Occurs when parsing string into integer value.
    #[error("{0}")]
    ParseInt(#[from] ParseIntError),

    /// Occurs when parsing string into float value.
    #[error("{0}")]
    ParseFloat(#[from] ParseFloatError),

    /// Any error when converting. You can use this wrapper to implement your original conversion
    /// methods.
    #[error(transparent)]
    Other(Box<dyn std::error::Error>),
}
