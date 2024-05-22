use super::*;

#[derive(Debug, Dynamodel, PartialEq)]
struct Outer {
    inner: Inner,
}

#[derive(Debug, Dynamodel, PartialEq)]
struct Inner {
    attr: String,
}

#[test]
fn test_into_hashmap() {
    let m = Outer {
        inner: Inner { attr: "foo".into() },
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
fn test_try_from_hashmap() {
    let expected = Outer {
        inner: Inner { attr: "foo".into() },
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
fn test_try_from_hashmap_field_not_set() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Outer::try_from(item);

    assert_field_not_set!(actual, "inner");
}

#[test]
fn test_try_from_hashmap_inner_field_not_set() {
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("inner".into(), AttributeValue::M(HashMap::new()));
    let actual = Outer::try_from(item);

    assert_field_not_set!(actual, "attr");
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
