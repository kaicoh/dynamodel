# dynamodel

[![Version](https://img.shields.io/crates/v/dynamodel)](https://crates.io/crates/dynamodel)
[![License](https://img.shields.io/crates/l/dynamodel)](LICENSE)
[![Test](https://img.shields.io/github/actions/workflow/status/kaicoh/dynamodel/ci.yml)](https://github.com/kaicoh/dynamodel/actions/workflows/ci.yml)

This library provides a derive macro to implement conversions between your object and `HashMap<String, AttributeValue>`.

## Usage

```rust
use dynamodel::Dynamodel;
use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;

// Using `Dynamodel` macro, you can implement both
// `From<your struct> for HashMap<String, AttributeValue>` and
// `TryFrom<HashMap<String, AttributeValue>> for your struct` traits.
#[derive(Dynamodel, Debug, Clone, PartialEq)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

let person = Person {
    first_name: "Kanji".into(),
    last_name: "Tanaka".into(),
    age: 23,
};

let item: HashMap<String, AttributeValue> = [
    ("first_name".to_string(), AttributeValue::S("Kanji".into())),
    ("last_name".to_string(), AttributeValue::S("Tanaka".into())),
    ("age".to_string(), AttributeValue::N("23".into()))
].into();

// Convert from Person into HashMap<String, AttributeValue>.
let converted: HashMap<String, AttributeValue> = person.clone().into();
assert_eq!(converted, item);

// Convert from HashMap<String, AttributeValue> into Person.
// This conversion uses std::convert::TryFrom trait, so this returns a Result.
let converted: Person = item.try_into().unwrap();
assert_eq!(converted, person);
```

## Implicit conversion

This macro implicitly converts some types, so you don't have to add any code. The types are as follows.

| Type | AttributeValue variant | Condition |
|---|---|---|
| `String` | `AttributeValue::S` | none |
| `u8, u16, u32, u64, u128, usize`<br>`i8, i16, i32, i64, i128, isize`<br>`f32, f64` | `AttributeValue::N` | none |
| `bool` | `AttributeValue::Bool` | none |
| Any structs or enums | `AttributeValue::M` | must implement both<br>`Into<HashMap<String, AttributeValue>>`<br>and<br>`TryFrom<HashMap<String, AttributeValue>, Error = ConvertError>` |
| `Vec<inner type>` | `AttributeValue::L` | the inner type must be one of the implicit conversion types. |
| `Option<inner type>` | Depends on the inner type | tye inner type must be one of the implicit conversion types. |

## Explicit conversion

Using the field attribute, you can implement custom conversion methods for any type like this.

```rust
use dynamodel::{Dynamodel, ConvertError};
use std::collections::HashMap;
use aws_sdk_dynamodb::{types::AttributeValue, primitives::Blob};

// Vec<u8> is converted to AttributeValue::L by default,
// but this case, the `data` field is converted to AttributeValue::B.
#[derive(Dynamodel)]
struct BinaryData {
    #[dynamodel(into = "to_blob", try_from = "from_blob")]
    data: Vec<u8>
}

fn to_blob(value: Vec<u8>) -> AttributeValue {
    AttributeValue::B(Blob::new(value))
}

fn from_blob(value: &AttributeValue) -> Result<Vec<u8>, ConvertError> {
    value.as_b()
        .map(|b| b.clone().into_inner())
        .map_err(|err| ConvertError::AttributeValueUnmatched("B".to_string(), err.clone()))
}
```

The function definition must meet these conditions.

| Field attribute | Argument | Return |
|---|---|---|
| `#[dynamodel(into = "...")]`| `field type` | `AttributeValue` |
| `#[dynamodel(try_from = "...")]` | `&AttributeValue` | `Result<field type, ConvertError>` |

## Example

### Single-table design

The following diagram shows that both `Video` and `VideoStats` are stored in the same table.

![videos table](https://github.com/kaicoh/dynamodel/blob/images/videos_table.png?raw=true)

```rust
#[derive(Dynamodel)]
#[dynamodel(extra = "VideoStats::sort_key", rename_all = "PascalCase")]
struct VideoStats {
    #[dynamodel(rename = "PK")]
    id: String,
    view_count: u64,
}

impl VideoStats {
    fn sort_key(&self) -> HashMap<String, AttributeValue> {
        [
            ("SK".to_string(), AttributeValue::S("VideoStats".into())),
        ].into()
    }
}
```

And suppose you want to add a `VideoComment` object that is sortable by timestamp, like this.

![video comments object](https://github.com/kaicoh/dynamodel/blob/images/videos_table_2.png?raw=true)

```rust
#[derive(Dynamodel)]
#[dynamodel(rename_all = "PascalCase")]
struct VideoComment {
    #[dynamodel(rename = "PK")]
    id: String,
    #[dynamodel(rename = "SK", into = "sort_key", try_from = "get_timestamp")]
    timestamp: String,
    content: String,
}

fn sort_key(timestamp: String) -> AttributeValue {
    AttributeValue::S(format!("VideoComment#{timestamp}"))
}

fn get_timestamp(value: &AttributeValue) -> Result<String, ConvertError> {
    value.as_s()
        .map(|v| v.split('#').last().unwrap().to_string())
        .map_err(|e| ConvertError::AttributeValueUnmatched("S".into(), e.clone()))
}
```

## More features

For more features, refer to [this wiki](https://github.com/kaicoh/dynamodel/wiki).

## License

This software is released under the [MIT License](LICENSE).
