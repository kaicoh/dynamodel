use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum NewType {
    Strs(Vec<String>),
}

#[test]
fn test_into_hashmap() {
    let val = NewType::Strs(vec!["foo".into(), "bar".into()]);
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> = [(
        "Strs".to_string(),
        AttributeValue::L(vec![
            AttributeValue::S("foo".into()),
            AttributeValue::S("bar".into()),
        ]),
    )]
    .into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_empty_vector() {
    let val = NewType::Strs(vec![]);
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> =
        [("Strs".to_string(), AttributeValue::L(vec![]))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = NewType::Strs(vec!["foo".into(), "bar".into()]);

    let item: HashMap<String, AttributeValue> = [(
        "Strs".to_string(),
        AttributeValue::L(vec![
            AttributeValue::S("foo".into()),
            AttributeValue::S("bar".into()),
        ]),
    )]
    .into();
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
        [("Strs".to_string(), AttributeValue::S("foo".into()))].into();

    let actual = NewType::try_from(item);

    assert_attribute_unmatch!(actual, "L");

    let item: HashMap<String, AttributeValue> = [(
        "Strs".to_string(),
        AttributeValue::L(vec![AttributeValue::N("10".into())]),
    )]
    .into();

    let actual = NewType::try_from(item);

    assert_attribute_unmatch!(actual, "S");
}
