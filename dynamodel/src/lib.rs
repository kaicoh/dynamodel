//! # dynamodel
//!
//! This library provides a derive macro to implement conversions between your object and
//! [HashMap](std::collections::HashMap)<[String], [AttributeValue]>.
//!
//! ## Derive macro [`Dynamodel`]
//!
//! The [`Dynamodel`] derive macro implements these three traits to use
//! [`aws-sdk-dynamodb`](https://crates.io/crates/aws-sdk-dynamodb) more comfortably.
//!
//! * `Into<HashMap<String, AttributeValue>>`
//! * `TryFrom<HashMap<String, AttributeValue>>`
//! * The [`AttributeValueConvertible`] trait enables the types that implement it to be converted from and to `AttributeValue`.
//!
//! ```ignore
//! #[derive(Dynamodel)]        Convertible
//! struct YourStruct { ... }  <===========>  HashMap<String, AttributeValue>
//!
//! #[derive(Dynamodel)]    Convertible
//! enum YourEnum { ... }  <===========>  HashMap<String, AttributeValue>
//! ```
//!
//! ### Requirements to use [`Dynamodel`]
//!
//! To use the [`Dynamodel`] macro, all types of your object's fields must implement
//! the `AttributeValueConvertible` trait.
//!
//! By default, these types automatically implement the [`AttributeValueConvertible`]
//! trait, so no additional code is required when using these types.
//!
//! | Type | `AttributeValue` variant |
//! |---|---|
//! | `String` | `AttributeValue::S("...")` |
//! | `u8, u16, u32, u64, u128, usize`<br>`i8, i16, i32, i64, i128, isize`<br>`f32, f64` | `AttributeValue::N("...")` |
//! | `bool` | `AttributeValue::Bool(...)` |
//! | `Vec` of any types that implement `AttributeValueConvertible` | `AttributeValue::L([...])` |
//! | Any types that implement `Dynamodel` macro | `AttributeValue::M({ ... })` |
//!
//! The last row of the above table shows that once you apply the [`Dynamodel`] macro to your object,
//! it also implements the [`AttributeValueConvertible`] trait for your object.
//!
//! **So, you can create nested structures of objects that apply the [`Dynamodel`] macro.**
//!
//! If you want to use additional types, you need to implement the `AttributeValueConvertible`
//! trait for your type.
//!
//! ## Usage
//!
//! ```rust
//! use dynamodel::Dynamodel;
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::types::AttributeValue;
//!
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
//! ### Modifying the default behavior
//!
//! Like the [`Serde`](https://crates.io/crates/serde) crate, you can modify the default behavior
//! through attributes like this.
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
pub trait AttributeValueConvertible: Sized {
    fn into_attribute_value(self) -> AttributeValue;
    fn try_from_attribute_value(value: &AttributeValue) -> Result<Self, ConvertError>;
}

impl AttributeValueConvertible for String {
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::S(self)
    }
    fn try_from_attribute_value(value: &AttributeValue) -> Result<Self, ConvertError> {
        value
            .as_s()
            .map(|v| v.to_string())
            .map_err(unmatch_err("S"))
    }
}

impl AttributeValueConvertible for bool {
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Bool(self)
    }
    fn try_from_attribute_value(value: &AttributeValue) -> Result<Self, ConvertError> {
        value.as_bool().copied().map_err(unmatch_err("Bool"))
    }
}

macro_rules! impl_to_nums {
    ($($ty:ty),*) => {
        $(
            impl AttributeValueConvertible for $ty {
                fn into_attribute_value(self) -> AttributeValue {
                    AttributeValue::N(self.to_string())
                }
                fn try_from_attribute_value(value: &AttributeValue) -> Result<Self, ConvertError> {
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

impl<T: AttributeValueConvertible> AttributeValueConvertible for Vec<T> {
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::L(
            self.into_iter()
                .map(AttributeValueConvertible::into_attribute_value)
                .collect(),
        )
    }
    fn try_from_attribute_value(value: &AttributeValue) -> Result<Self, ConvertError> {
        let mut values: Vec<T> = vec![];
        for v in value.as_l().map_err(unmatch_err("L"))?.iter() {
            let v: T = AttributeValueConvertible::try_from_attribute_value(v)?;
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
            AttributeValueConvertible::try_from_attribute_value(&value);
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
        let result: Result<bool, ConvertError> =
            AttributeValueConvertible::try_from_attribute_value(&value);
        assert!(!result.unwrap());
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
            AttributeValueConvertible::try_from_attribute_value(&value);
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
            AttributeValueConvertible::try_from_attribute_value(&value);
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
                        let result: Result<$ty, ConvertError> = AttributeValueConvertible::try_from_attribute_value(&value);
                        assert_eq!(result.unwrap(), expected);
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
                        let result: Result<Vec<$ty>, ConvertError> = AttributeValueConvertible::try_from_attribute_value(&value);
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
                        let result: Result<$ty, ConvertError> = AttributeValueConvertible::try_from_attribute_value(&value);
                        assert_eq!(result.unwrap(), expected);
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
                        let result: Result<Vec<$ty>, ConvertError> = AttributeValueConvertible::try_from_attribute_value(&value);
                        assert_eq!(result.unwrap(), expected);
                    }
                }
            )*
        }
    }

    test_float! { f32, f64 }
}
