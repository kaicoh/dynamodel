use super::*;

#[derive(Debug, Dynamodel, PartialEq)]
struct Model {
    attr: Vec<String>,
}

#[test]
fn test_into_hashmap() {
    let m = Model {
        attr: vec!["foo".into(), "bar".into()],
    };
    let actual: HashMap<String, AttributeValue> = m.into();

    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert(
        "attr".into(),
        AttributeValue::L(vec![
            AttributeValue::S("foo".into()),
            AttributeValue::S("bar".into()),
        ]),
    );

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_empty_vector() {
    let m = Model { attr: vec![] };
    let actual: HashMap<String, AttributeValue> = m.into();
    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert("attr".into(), AttributeValue::L(vec![]));

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Model {
        attr: vec!["foo".into(), "bar".into()],
    };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "attr".into(),
        AttributeValue::L(vec![
            AttributeValue::S("foo".into()),
            AttributeValue::S("bar".into()),
        ]),
    );
    let actual = Model::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_into_empty_vector() {
    let expected = Model { attr: vec![] };
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("attr".into(), AttributeValue::L(vec![]));
    let actual = Model::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_field_not_set() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Model::try_from(item);

    assert_field_not_set!(actual, "attr");
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("attr".into(), AttributeValue::N("10".into()));
    let actual = Model::try_from(item);

    assert_attribute_unmatch!(actual, "L");

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "attr".into(),
        AttributeValue::L(vec![AttributeValue::N("10".into())]),
    );
    let actual = Model::try_from(item);

    assert_attribute_unmatch!(actual, "S");
}
