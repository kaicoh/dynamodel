use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum NewType {
    Str(String),
}

#[test]
fn test_into_hashmap() {
    let val = NewType::Str("foo".into());
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> =
        [("Str".to_string(), AttributeValue::S("foo".into()))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = NewType::Str("foo".into());

    let item: HashMap<String, AttributeValue> =
        [("Str".to_string(), AttributeValue::S("foo".into()))].into();

    let actual = NewType::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_variant_not_found() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = NewType::try_from(item);

    assert_variant_not_found!(actual);
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let item: HashMap<String, AttributeValue> =
        [("Str".to_string(), AttributeValue::N("10".into()))].into();

    let actual = NewType::try_from(item);

    assert_attribute_unmatch!(actual, "S");
}
