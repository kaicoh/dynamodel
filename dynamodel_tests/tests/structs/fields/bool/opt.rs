use super::*;

#[derive(Debug, Dynamodel, PartialEq)]
struct Model {
    attr: Option<bool>,
}

#[test]
fn test_into_hashmap() {
    let m = Model { attr: Some(true) };
    let actual: HashMap<String, AttributeValue> = m.into();

    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert("attr".into(), AttributeValue::Bool(true));

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_none() {
    let m = Model { attr: None };
    let actual: HashMap<String, AttributeValue> = m.into();
    let expected: HashMap<String, AttributeValue> = HashMap::new();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Model { attr: Some(true) };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("attr".into(), AttributeValue::Bool(true));
    let actual = Model::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_into_none() {
    let expected = Model { attr: None };
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Model::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("attr".into(), AttributeValue::N("10".into()));
    let actual = Model::try_from(item);

    assert_attribute_unmatch!(actual, "Bool");
}
