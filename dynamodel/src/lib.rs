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
//! Using field attribute, you can implement original conversion methods for any types like this.
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
//!
//! ## Rename HashMap key
//!
//! Like [serde crate](https://crates.io/crates/serde), you can rename
//! [HashMap](std::collections::HashMap) key from your struct field name.
//!
//! ### Container attribute `rename_all`
//!
//! The allowed values for `rename_all` attribute are `UPPERCASE`, `PascalCase`, `camelCase`,
//! `SCREAMING_SNAKE_CASE`, `kebab-case` and `SCREAMING-KEBAB-CASE`.
//!
//! ```rust
//! use dynamodel::Dynamodel;
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::types::AttributeValue;
//!
//! // Use `rename_all` as container attribute.
//! #[derive(Dynamodel, Debug, Clone, PartialEq)]
//! #[dynamodel(rename_all = "PascalCase")]
//! struct Person {
//!     first_name: String,
//!     last_name: String,
//! }
//!
//! let person = Person {
//!     first_name: "Kanji".into(),
//!     last_name: "Tanaka".into(),
//! };
//!
//! let item: HashMap<String, AttributeValue> = [
//!     ("FirstName".to_string(), AttributeValue::S("Kanji".into())),
//!     ("LastName".to_string(), AttributeValue::S("Tanaka".into())),
//! ].into();
//!
//! let converted: HashMap<String, AttributeValue> = person.clone().into();
//! assert_eq!(converted, item);
//!
//! let converted: Person = item.try_into().unwrap();
//! assert_eq!(converted, person);
//! ```
//!
//! ### Field attribute `rename`
//!
//! You can also rename key using field attribute `rename`.
//!
//! ```rust
//! use dynamodel::Dynamodel;
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::types::AttributeValue;
//!
//! #[derive(Dynamodel, Debug, Clone, PartialEq)]
//! struct Person {
//!     // Use `rename` as field attribute.
//!     #[dynamodel(rename = "GivenName")]
//!     first_name: String,
//!     #[dynamodel(rename = "FamilyName")]
//!     last_name: String,
//! }
//!
//! let person = Person {
//!     first_name: "Kanji".into(),
//!     last_name: "Tanaka".into(),
//! };
//!
//! let item: HashMap<String, AttributeValue> = [
//!     ("GivenName".to_string(), AttributeValue::S("Kanji".into())),
//!     ("FamilyName".to_string(), AttributeValue::S("Tanaka".into())),
//! ].into();
//!
//! let converted: HashMap<String, AttributeValue> = person.clone().into();
//! assert_eq!(converted, item);
//!
//! let converted: Person = item.try_into().unwrap();
//! assert_eq!(converted, person);
//! ```
//!
//! ## Extra key-value pair for HashMap
//!
//! If you design your struct according to the
//! [single-table design](https://aws.amazon.com/jp/blogs/compute/creating-a-single-table-design-with-amazon-dynamodb/),
//! you want set additional key-value sets to the [HashMap](std::collections::HashMap) sometimes.
//!
//! For example, the following diagram shows both `Video` and `VideoStats` are stored in the same
//! table.
//!
//! ![Videos table](https://github.com/kaicoh/dynamodel/blob/images/videos_table.png?raw=true)
//!
//! Using container attribute `extra`, you can implement this structure.
//! The `extra` value must be a path for function whose argument is a reference of the struct's
//! instance and its return type is [HashMap](std::collections::HashMap)<[String],
//! [AttributeValue]>.
//!
//! ```rust
//! use dynamodel::Dynamodel;
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::types::AttributeValue;
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
//! let converted: HashMap<String, AttributeValue> = stats.into();
//! assert_eq!(converted, item);
//! ```
//!
//! ## Skip setting key-value pair to HashMap and get field value from HashMap itself
//!
//! For example, suppose you want to add `VideoComment` sortable by timestamp like this.
//!
//! ![Videos table](https://github.com/kaicoh/dynamodel/blob/images/videos_table_2.png?raw=true)
//!
//! And the struct is following.
//!
//! ```rust
//! struct VideoComment {
//!     id: String,
//!     content: String,
//!     timestamp: String,
//! }
//! ```
//!
//! This time, when converting from Struct to HashMap you must not set `timestamp` as key,
//! but when converting from HashMap to Struct, you must set `timestamp` field from sort key.
//!
//! In this case, you can use `skip_into` and `try_from_item` attributes. The field having
//! `skip_into` attribute is ignored when setting key-value pair to HashMap and the one having
//! `try_from_item` attribute can be set its value from HashMap itself not from AttributeValue
//! like `try_from` attribute.
//!
//! | Field Attribute | Argument | Return |
//! |---|---|---|
//! | `try_from` | `&`[`AttributeValue`] | [`Result`]`<field type,`[`ConvertError`]`>` |
//! | `try_from_item` | `&`[`HashMap`](std::collections::HashMap)`<String,`[`AttributeValue`]`>` | [`Result`]`<field type,`[`ConvertError`]`>` |
//!
//! ```rust
//! use dynamodel::{Dynamodel, ConvertError};
//! # use std::collections::HashMap;
//! # use aws_sdk_dynamodb::types::AttributeValue;
//!
//! #[derive(Dynamodel, Debug, Clone, PartialEq)]
//! #[dynamodel(extra = "VideoComment::sort_key")]
//! struct VideoComment {
//!     #[dynamodel(rename = "PK")]
//!     id: String,
//!     content: String,
//!     // Using `skip_into` attribute, you can skip setting `timestamp` value to HashMap
//!     // and `try_from_item` attribute enables retrieving field value from HashMap itself.
//!     #[dynamodel(skip_into, try_from_item = "get_timestamp")]
//!     timestamp: String
//! }
//!
//! impl VideoComment {
//!     fn sort_key(&self) -> HashMap<String, AttributeValue> {
//!         [
//!             (
//!                 "SK".to_string(),
//!                 AttributeValue::S(format!("VideoComment#{}", self.timestamp)),
//!             ),
//!         ].into()
//!     }
//! }
//!
//! fn get_timestamp(item: &HashMap<String, AttributeValue>) -> Result<String, ConvertError> {
//!     item.get("SK")
//!         .ok_or(ConvertError::FieldNotSet("SK".into()))
//!         .and_then(|v| v.as_s().map_err(|e| ConvertError::AttributeValueUnmatched("S".into(), e.clone())))
//!         .map(|v| v.split('#').last().unwrap().to_string())
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
//!     ("content".to_string(), AttributeValue::S("Good video!".into())),
//! ].into();
//!
//! let converted: HashMap<String, AttributeValue> = comment.clone().into();
//! assert_eq!(converted, item);
//!
//! let converted: VideoComment = item.try_into().unwrap();
//! assert_eq!(converted, comment);
//! ```

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
