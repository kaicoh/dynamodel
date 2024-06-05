//! # dynamodel
//!
//! This library provides a derive macro to implement conversions between your object and
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
//! This macro implicitly converts some types, so you don't have to add any code. The types are
//! as follows.
//!
//! | Type | AttributeValue variant | Condition |
//! |---|---|---|
//! | `String` | `AttributeValue::S` | none |
//! | `u8, u16, u32, u64, u128, usize`<br>`i8, i16, i32, i64, i128, isize`<br>`f32, f64` | `AttributeValue::N` | none |
//! | `bool` | `AttributeValue::Bool` | none |
//! | Any structs or enums | `AttributeValue::M` | must implement both<br>`Into<HashMap<String, AttributeValue>>`<br>and<br>`TryFrom<HashMap<String, AttributeValue>, Error = ConvertError>` |
//! | `Vec<inner type>` | `AttributeValue::L` | the inner type must be one of the implicit conversion types. |
//! | `Option<inner type>` | Depends on the inner type | the inner type must be one of the implicit conversion types. |
//!
//! ## Explicit conversion
//!
//! Using the field attribute, you can implement custom conversion methods for any type like this.
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
//! The function definition must meet these conditions.
//!
//! | Field attribute | Argument | Return |
//! |---|---|---|
//! | `#[dynamodel(into = "...")]`| `field type` | `AttributeValue` |
//! | `#[dynamodel(try_from = "...")]` | `&AttributeValue` | `Result<field type, ConvertError>` |
//!
//! ## Example
//!
//! ### Single-table design
//!
//! The following diagram shows that both `Video` and `VideoStats` are stored in the same table.
//!
//! ![videos table](https://github.com/kaicoh/dynamodel/blob/images/videos_table.png?raw=true)
//!
//! ```rust
//! use dynamodel::{Dynamodel, ConvertError};
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::{types::AttributeValue, primitives::Blob};
//!
//! #[derive(Dynamodel, Debug, Clone, PartialEq)]
//! #[dynamodel(extra = "VideoStats::sort_key", rename_all = "PascalCase")]
//! struct VideoStats {
//!     #[dynamodel(rename = "PK")]
//!     id: String,
//!     view_count: u64,
//! }
//!
//! impl VideoStats {
//!     fn sort_key(&self) -> HashMap<String, AttributeValue> {
//!         [
//!             ("SK".to_string(), AttributeValue::S("VideoStats".into())),
//!         ].into()
//!     }
//! }
//!
//! let stats = VideoStats {
//!     id: "7cf27a02".into(),
//!     view_count: 147,
//! };
//!
//! let item: HashMap<String, AttributeValue> = [
//!     ("PK".to_string(), AttributeValue::S("7cf27a02".into())),
//!     ("SK".to_string(), AttributeValue::S("VideoStats".into())),
//!     ("ViewCount".to_string(), AttributeValue::N("147".into())),
//! ].into();
//!
//! let converted: HashMap<String, AttributeValue> = stats.clone().into();
//! assert_eq!(converted, item);
//!
//! let converted: VideoStats = item.try_into().unwrap();
//! assert_eq!(converted, stats);
//! ```
//!
//! And suppose you want to add a `VideoComment` object that is sortable by timestamp,
//! like this.
//!
//! ![video comments object](https://github.com/kaicoh/dynamodel/blob/images/videos_table_2.png?raw=true)
//!
//! ```rust
//! use dynamodel::{Dynamodel, ConvertError};
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::{types::AttributeValue, primitives::Blob};
//!
//! #[derive(Dynamodel, Debug, Clone, PartialEq)]
//! #[dynamodel(rename_all = "PascalCase")]
//! struct VideoComment {
//!     #[dynamodel(rename = "PK")]
//!     id: String,
//!     #[dynamodel(rename = "SK", into = "sort_key", try_from = "get_timestamp")]
//!     timestamp: String,
//!     content: String,
//! }
//!
//! fn sort_key(timestamp: String) -> AttributeValue {
//!     AttributeValue::S(format!("VideoComment#{timestamp}"))
//! }
//!
//! fn get_timestamp(value: &AttributeValue) -> Result<String, ConvertError> {
//!     value.as_s()
//!         .map(|v| v.split('#').last().unwrap().to_string())
//!         .map_err(|e| ConvertError::AttributeValueUnmatched("S".into(), e.clone()))
//! }
//!
//! let comment = VideoComment {
//!     id: "7cf27a02".into(),
//!     content: "Good video!".into(),
//!     timestamp: "2023-04-05T12:34:56".into(),
//! };
//!
//! let item: HashMap<String, AttributeValue> = [
//!     ("PK".to_string(), AttributeValue::S("7cf27a02".into())),
//!     ("SK".to_string(), AttributeValue::S("VideoComment#2023-04-05T12:34:56".into())),
//!     ("Content".to_string(), AttributeValue::S("Good video!".into())),
//! ].into();
//!
//! let converted: HashMap<String, AttributeValue> = comment.clone().into();
//! assert_eq!(converted, item);
//!
//! let converted: VideoComment = item.try_into().unwrap();
//! assert_eq!(converted, comment);
//! ```
//!
//! ## More features
//!
//! For more features, refer to [this wiki](https://github.com/kaicoh/dynamodel/wiki).

/// Derive macro to implement both `Into<HashMap<String, AttributeValue>>` and `TryFrom<HashMap<String, AttributeValue>>` traits.
///
/// For details, refer to [the wiki](https://github.com/kaicoh/dynamodel/wiki).
pub use dynamodel_derive::Dynamodel;

use aws_sdk_dynamodb::types::AttributeValue;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/// An error occurs when converting from a `HashMap<String, AttributeValue>` to your object.
#[derive(Debug, Error)]
pub enum ConvertError {
    /// There is no key-value pair for this field in the HashMap.
    #[error("`{0}` field is not set")]
    FieldNotSet(String),

    /// There is a key-value pair for the field, but the type of AttributeValue is not what is expected.
    #[error("expect `{0}` type, but got `{1:?}`")]
    AttributeValueUnmatched(String, AttributeValue),

    /// The value in the HashMap should be an integer, but it isn't.
    #[error("{0}")]
    ParseInt(#[from] ParseIntError),

    /// The value in the HashMap should be a float, but it isn't.
    #[error("{0}")]
    ParseFloat(#[from] ParseFloatError),

    /// There are no vairants for the enum in the HashMap.
    #[error("not found any variant in hashmap")]
    VariantNotFound,

    /// Any other errors when converting from a HashMap to your object.
    /// You can wrap your original errors in this variant.
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

fn unmatch_err(expected: &str) -> impl Fn(&AttributeValue) -> ConvertError + '_ {
    |value: &AttributeValue| {
        ConvertError::AttributeValueUnmatched(expected.to_string(), value.to_owned())
    }
}

/// Types that implement this trait on objects with the [`Dynamodel`] macro can be
/// implicitly converted from and into [`AttributeValue`].
pub trait AttributeValuable: Sized {
    fn into_attribute_value(self) -> AttributeValue;
    fn try_from_attribute_value(value: AttributeValue) -> Result<Self, ConvertError>;
}

impl AttributeValuable for String {
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::S(self)
    }
    fn try_from_attribute_value(value: AttributeValue) -> Result<Self, ConvertError> {
        value
            .as_s()
            .map(|v| v.to_string())
            .map_err(unmatch_err("S"))
    }
}

impl AttributeValuable for bool {
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Bool(self)
    }
    fn try_from_attribute_value(value: AttributeValue) -> Result<Self, ConvertError> {
        value.as_bool().copied().map_err(unmatch_err("Bool"))
    }
}

macro_rules! impl_to_nums {
    ($($ty:ty),*) => {
        $(
            impl AttributeValuable for $ty {
                fn into_attribute_value(self) -> AttributeValue {
                    AttributeValue::N(self.to_string())
                }
                fn try_from_attribute_value(value: AttributeValue) -> Result<Self, ConvertError> {
                    value.as_n()
                        .map_err(unmatch_err("N"))
                        .and_then(|v| v.parse::<$ty>().map_err(|e| e.into()))
                }
            }
         )*
    }
}

impl_to_nums! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
}

impl<T: AttributeValuable> AttributeValuable for Option<T> {
    fn into_attribute_value(self) -> AttributeValue {
        self.map(AttributeValuable::into_attribute_value)
            .unwrap_or(AttributeValue::Null(true))
    }
    fn try_from_attribute_value(value: AttributeValue) -> Result<Self, ConvertError> {
        match value {
            AttributeValue::Null(_) => Ok(None),
            _ => {
                let result: Result<T, ConvertError> =
                    AttributeValuable::try_from_attribute_value(value);
                result.map(|v| Some(v))
            }
        }
    }
}

impl<T: AttributeValuable> AttributeValuable for Vec<T> {
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::L(
            self.into_iter()
                .map(AttributeValuable::into_attribute_value)
                .collect(),
        )
    }
    fn try_from_attribute_value(value: AttributeValue) -> Result<Self, ConvertError> {
        let mut values: Vec<T> = vec![];
        for v in value.as_l().map_err(unmatch_err("L"))?.iter().cloned() {
            let v: T = AttributeValuable::try_from_attribute_value(v)?;
            values.push(v);
        }
        Ok(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_can_be_converted_into_attribute_value() {
        let value = "Hello".to_string();
        assert_eq!(
            value.into_attribute_value(),
            AttributeValue::S("Hello".into())
        );
    }

    #[test]
    fn string_can_be_converted_from_attribute_value() {
        let value = AttributeValue::S("Hello".into());
        let result: Result<String, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), "Hello".to_string());
    }

    #[test]
    fn boolean_can_be_converted_into_attribute_value() {
        let value = true;
        assert_eq!(value.into_attribute_value(), AttributeValue::Bool(true));
    }

    #[test]
    fn boolean_can_be_converted_from_attribute_value() {
        let value = AttributeValue::Bool(false);
        let result: Result<bool, ConvertError> = AttributeValuable::try_from_attribute_value(value);
        assert!(!result.unwrap());
    }

    #[test]
    fn optional_string_can_be_converted_into_attribute_value() {
        let value = Some("Hello".to_string());
        assert_eq!(
            value.into_attribute_value(),
            AttributeValue::S("Hello".into())
        );

        let value: Option<String> = None;
        assert_eq!(value.into_attribute_value(), AttributeValue::Null(true));
    }

    #[test]
    fn optional_string_can_be_converted_from_attribute_value() {
        let value = AttributeValue::S("Hello".into());
        let result: Result<Option<String>, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), Some("Hello".to_string()));

        let value = AttributeValue::Null(true);
        let result: Result<Option<String>, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn optional_boolean_can_be_converted_into_attribute_value() {
        let value = Some(false);
        assert_eq!(value.into_attribute_value(), AttributeValue::Bool(false));

        let value: Option<bool> = None;
        assert_eq!(value.into_attribute_value(), AttributeValue::Null(true));
    }

    #[test]
    fn optional_boolean_can_be_converted_from_attribute_value() {
        let value = AttributeValue::Bool(true);
        let result: Result<Option<bool>, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), Some(true));

        let value = AttributeValue::Null(true);
        let result: Result<Option<bool>, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn string_vector_can_be_converted_into_attribute_value() {
        let value = vec!["Hello".to_string(), "World".to_string()];
        assert_eq!(
            value.into_attribute_value(),
            AttributeValue::L(vec![
                AttributeValue::S("Hello".into()),
                AttributeValue::S("World".into())
            ]),
        );
    }

    #[test]
    fn string_vector_can_be_converted_from_attribute_value() {
        let expected = vec!["Hello".to_string(), "World".to_string()];
        let value = AttributeValue::L(vec![
            AttributeValue::S("Hello".into()),
            AttributeValue::S("World".into()),
        ]);
        let result: Result<Vec<String>, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn boolean_vector_can_be_converted_into_attribute_value() {
        let value = vec![true, false];
        assert_eq!(
            value.into_attribute_value(),
            AttributeValue::L(vec![
                AttributeValue::Bool(true),
                AttributeValue::Bool(false)
            ]),
        );
    }

    #[test]
    fn boolean_vector_can_be_converted_from_attribute_value() {
        let expected = vec![true, false];
        let value = AttributeValue::L(vec![
            AttributeValue::Bool(true),
            AttributeValue::Bool(false),
        ]);
        let result: Result<Vec<bool>, ConvertError> =
            AttributeValuable::try_from_attribute_value(value);
        assert_eq!(result.unwrap(), expected);
    }

    macro_rules! test_int {
        ($($ty:ty),*) => {
            $(
                paste::item! {
                    #[test]
                    fn [<$ty _can_be_converted_into_attribute_value>]() {
                        let value: $ty = 10;
                        assert_eq!(value.into_attribute_value(), AttributeValue::N("10".into()));
                    }

                    #[test]
                    fn [<$ty _can_be_converted_from_attribute_value>]() {
                        let expected: $ty = 10;
                        let value = AttributeValue::N("10".into());
                        let result: Result<$ty, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), expected);
                    }

                    #[test]
                    fn [<optional_ $ty _can_be_converted_into_attribute_value>]() {
                        let value: Option<$ty> = Some(10);
                        assert_eq!(value.into_attribute_value(), AttributeValue::N("10".into()));

                        let value: Option<$ty> = None;
                        assert_eq!(value.into_attribute_value(), AttributeValue::Null(true));
                    }

                    #[test]
                    fn [<optional_ $ty _can_be_converted_from_attribute_value>]() {
                        let expected: Option<$ty> = Some(10);
                        let value = AttributeValue::N("10".into());
                        let result: Result<Option<$ty>, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), expected);

                        let value = AttributeValue::Null(true);
                        let result: Result<Option<$ty>, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), None);
                    }

                    #[test]
                    fn [<$ty _vector_can_be_converted_into_attribute_value>]() {
                        let value: Vec<$ty> = vec![10, 20];
                        assert_eq!(
                            value.into_attribute_value(),
                            AttributeValue::L(vec![AttributeValue::N("10".into()), AttributeValue::N("20".into())]),
                        );
                    }

                    #[test]
                    fn [<$ty _vector_can_be_converted_from_attribute_value>]() {
                        let expected: Vec<$ty> = vec![10, 20];
                        let value = AttributeValue::L(vec![AttributeValue::N("10".into()), AttributeValue::N("20".into())]);
                        let result: Result<Vec<$ty>, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), expected);
                    }
                }
            )*
        }
    }

    test_int! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

    macro_rules! test_float {
        ($($ty:ty),*) => {
            $(
                paste::item! {
                    #[test]
                    fn [<$ty _can_be_converted_into_attribute_value>]() {
                        let value: $ty = 1.2;
                        assert_eq!(value.into_attribute_value(), AttributeValue::N("1.2".into()));
                    }

                    #[test]
                    fn [<$ty _can_be_converted_from_attribute_value>]() {
                        let expected: $ty = 1.2;
                        let value = AttributeValue::N("1.2".into());
                        let result: Result<$ty, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), expected);
                    }

                    #[test]
                    fn [<optional_ $ty _can_be_converted_into_attribute_value>]() {
                        let value: Option<$ty> = Some(1.2);
                        assert_eq!(value.into_attribute_value(), AttributeValue::N("1.2".into()));

                        let value: Option<$ty> = None;
                        assert_eq!(value.into_attribute_value(), AttributeValue::Null(true));
                    }

                    #[test]
                    fn [<optional_ $ty _can_be_converted_from_attribute_value>]() {
                        let expected: Option<$ty> = Some(1.2);
                        let value = AttributeValue::N("1.2".into());
                        let result: Result<Option<$ty>, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), expected);

                        let value = AttributeValue::Null(true);
                        let result: Result<Option<$ty>, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), None);
                    }

                    #[test]
                    fn [<$ty _vector_can_be_converted_into_attribute_value>]() {
                        let value: Vec<$ty> = vec![1.2, 3.45];
                        assert_eq!(
                            value.into_attribute_value(),
                            AttributeValue::L(vec![AttributeValue::N("1.2".into()), AttributeValue::N("3.45".into())]),
                        );
                    }

                    #[test]
                    fn [<$ty _vector_can_be_converted_from_attribute_value>]() {
                        let expected: Vec<$ty> = vec![1.2, 3.45];
                        let value = AttributeValue::L(vec![AttributeValue::N("1.2".into()), AttributeValue::N("3.45".into())]);
                        let result: Result<Vec<$ty>, ConvertError> = AttributeValuable::try_from_attribute_value(value);
                        assert_eq!(result.unwrap(), expected);
                    }
                }
            )*
        }
    }

    test_float! { f32, f64 }
}
