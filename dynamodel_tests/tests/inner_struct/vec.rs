use super::*;

#[derive(Debug, Dynamodel, PartialEq)]
struct Outer {
    inner: Vec<Inner>,
}

#[derive(Debug, Dynamodel, PartialEq)]
struct Inner {
    attr: String,
}

#[test]
fn test_into_hashmap() {
    let m = Outer {
        inner: vec![Inner { attr: "foo".into() }],
    };
    let actual: HashMap<String, AttributeValue> = m.into();

    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert(
        "inner".into(),
        AttributeValue::L(vec![AttributeValue::M(Inner { attr: "foo".into() }.into())]),
    );

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_empty_vector() {
    let m = Outer { inner: vec![] };
    let actual: HashMap<String, AttributeValue> = m.into();
    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert("inner".into(), AttributeValue::L(vec![]));

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Outer {
        inner: vec![Inner { attr: "foo".into() }],
    };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "inner".into(),
        AttributeValue::L(vec![AttributeValue::M(Inner { attr: "foo".into() }.into())]),
    );
    let actual = Outer::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_into_empty_vector() {
    let expected = Outer { inner: vec![] };
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("inner".into(), AttributeValue::L(vec![]));
    let actual = Outer::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_field_not_set() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Outer::try_from(item);

    assert_field_not_set!(actual, "inner");
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("inner".into(), AttributeValue::N("10".into()));
    let actual = Outer::try_from(item);

    assert_attribute_unmatch!(actual, "L");
}

#[test]
fn test_try_from_hashmap_inner_unmatched_attribute_value() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "inner".into(),
        AttributeValue::L(vec![AttributeValue::N("10".into())]),
    );
    let actual = Outer::try_from(item);

    assert_attribute_unmatch!(actual, "M");
}
