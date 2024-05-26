use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum Message {
    Request { id: String, inner: Option<Inner> },
    Response { id: String, result: String },
}

#[derive(Debug, Dynamodel, PartialEq)]
struct Inner {
    attr: String,
}

#[test]
fn test_into_hashmap() {
    let msg = Message::Request {
        id: "1".into(),
        inner: Some(Inner { attr: "foo".into() }),
    };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        (
            "inner".to_string(),
            AttributeValue::M(Inner { attr: "foo".into() }.into()),
        ),
    ]
    .into();

    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_none() {
    let msg = Message::Request {
        id: "1".into(),
        inner: None,
    };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> =
        [("id".to_string(), AttributeValue::S("1".into()))].into();

    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Message::Request {
        id: "1".into(),
        inner: Some(Inner { attr: "foo".into() }),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        (
            "inner".to_string(),
            AttributeValue::M(Inner { attr: "foo".into() }.into()),
        ),
    ]
    .into();

    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_into_none() {
    let expected = Message::Request {
        id: "1".into(),
        inner: None,
    };

    let inner: HashMap<String, AttributeValue> =
        [("id".to_string(), AttributeValue::S("1".into()))].into();

    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_variant_not_found() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = Message::try_from(item);

    assert_variant_not_found!(actual);
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::S("foo".into()))].into();

    let actual = Message::try_from(item);

    assert_attribute_unmatch!(actual, "M");
}

#[test]
fn test_try_from_hashmap_inner_unmatched_attribute_value() {
    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("inner".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_attribute_unmatch!(actual, "M");
}
