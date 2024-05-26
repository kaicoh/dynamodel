use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum Message {
    Request { attr: Vec<bool> },
    Response { id: String, result: String },
}

#[test]
fn test_into_hashmap() {
    let msg = Message::Request {
        attr: vec![true, false],
    };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> = [(
        "attr".to_string(),
        AttributeValue::L(vec![
            AttributeValue::Bool(true),
            AttributeValue::Bool(false),
        ]),
    )]
    .into();

    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_empty_vector() {
    let msg = Message::Request { attr: vec![] };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> =
        [("attr".to_string(), AttributeValue::L(vec![]))].into();
    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Message::Request {
        attr: vec![true, false],
    };

    let inner: HashMap<String, AttributeValue> = [(
        "attr".to_string(),
        AttributeValue::L(vec![
            AttributeValue::Bool(true),
            AttributeValue::Bool(false),
        ]),
    )]
    .into();

    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_into_empty_vector() {
    let expected = Message::Request { attr: vec![] };

    let inner: HashMap<String, AttributeValue> =
        [("attr".to_string(), AttributeValue::L(vec![]))].into();

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
fn test_try_from_hashmap_field_not_set() {
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(HashMap::new()))].into();
    let actual = Message::try_from(item);

    assert_field_not_set!(actual, "attr");
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
    let inner: HashMap<String, AttributeValue> =
        [("attr".to_string(), AttributeValue::S("true".into()))].into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_attribute_unmatch!(actual, "L");

    let inner: HashMap<String, AttributeValue> = [(
        "attr".to_string(),
        AttributeValue::L(vec![AttributeValue::S("true".into())]),
    )]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_attribute_unmatch!(actual, "Bool");
}
