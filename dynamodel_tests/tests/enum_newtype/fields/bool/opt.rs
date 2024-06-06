use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum NewType {
    Bool(Option<bool>),
}

#[test]
fn test_into_hashmap() {
    let val = NewType::Bool(Some(true));
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> =
        [("Bool".to_string(), AttributeValue::Bool(true))].into();

    assert_eq!(actual, expected);

    let val = NewType::Bool(None);
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> =
        [("Bool".to_string(), AttributeValue::Null(true))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = NewType::Bool(Some(true));

    let item: HashMap<String, AttributeValue> =
        [("Bool".to_string(), AttributeValue::Bool(true))].into();

    let actual = NewType::try_from(item);

    assert_ok_eq!(actual, expected);

    let expected = NewType::Bool(None);

    let item: HashMap<String, AttributeValue> =
        [("Bool".to_string(), AttributeValue::Null(true))].into();

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
        [("Bool".to_string(), AttributeValue::N("10".into()))].into();

    let actual = NewType::try_from(item);

    assert_attribute_unmatch!(actual, "Bool");
}
