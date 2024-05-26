use super::*;

#[derive(Debug, Dynamodel, PartialEq)]
struct Outer {
    inner: Option<Inner>,
}

#[derive(Debug, Dynamodel, PartialEq)]
struct Inner {
    attr: String,
}

#[test]
fn test_into_hashmap() {
    let m = Outer {
        inner: Some(Inner { attr: "foo".into() }),
    };
    let actual: HashMap<String, AttributeValue> = m.into();

    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
    expected.insert(
        "inner".into(),
        AttributeValue::M(Inner { attr: "foo".into() }.into()),
    );

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_none() {
    let m = Outer { inner: None };
    let actual: HashMap<String, AttributeValue> = m.into();
    let expected: HashMap<String, AttributeValue> = HashMap::new();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Outer {
        inner: Some(Inner { attr: "foo".into() }),
    };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "inner".into(),
        AttributeValue::M(Inner { attr: "foo".into() }.into()),
    );
    let actual = Outer::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_into_none() {
    let expected = Outer { inner: None };
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Outer::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("inner".into(), AttributeValue::N("10".into()));
    let actual = Outer::try_from(item);

    assert_attribute_unmatch!(actual, "M");
}

#[test]
fn test_try_from_hashmap_inner_unmatched_attribute_value() {
    let mut inner: HashMap<String, AttributeValue> = HashMap::new();
    inner.insert("attr".into(), AttributeValue::N("10".into()));

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("inner".into(), AttributeValue::M(inner));
    let actual = Outer::try_from(item);

    assert_attribute_unmatch!(actual, "S");
}
