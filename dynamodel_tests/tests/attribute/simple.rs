use super::*;
use aws_sdk_dynamodb::primitives::Blob;

#[derive(Debug, Dynamodel, PartialEq)]
struct Model {
    #[dynamodel(into = "to_blob", try_from = "from_blob")]
    bytes: Vec<u8>,
}

fn to_blob(value: Vec<u8>) -> AttributeValue {
    AttributeValue::B(Blob::new(value))
}

fn from_blob(value: &AttributeValue) -> Result<Vec<u8>, ConvertError> {
    value
        .as_b()
        .map_err(|e| ConvertError::AttributeValueUnmatched("B".to_string(), e.clone()))
        .map(|b| b.clone().into_inner())
}

#[test]
fn test_into_hashmap() {
    let m = Model {
        bytes: b"hello".into(),
    };
    let actual: HashMap<String, AttributeValue> = m.into();

    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert("bytes".into(), AttributeValue::B(Blob::new(b"hello")));

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Model {
        bytes: b"hello".into(),
    };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("bytes".into(), AttributeValue::B(Blob::new(b"hello")));
    let actual = Model::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_field_not_set() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Model::try_from(item);

    assert_field_not_set!(actual, "bytes");
}

#[test]
fn test_try_from_hashmap_convertion_error() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("bytes".into(), AttributeValue::N("10".into()));
    let actual = Model::try_from(item);

    assert_attribute_unmatch!(actual, "B");
}
