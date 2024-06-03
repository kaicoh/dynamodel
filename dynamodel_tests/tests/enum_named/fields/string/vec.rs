use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum Message {
    Request { methods: Vec<String> },
    Response { id: String, result: String },
}

#[test]
fn test_into_hashmap() {
    let msg = Message::Request {
        methods: vec!["GET".into(), "POST".into()],
    };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> = [(
        "methods".to_string(),
        AttributeValue::L(vec![
            AttributeValue::S("GET".into()),
            AttributeValue::S("POST".into()),
        ]),
    )]
    .into();

    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_empty_vector() {
    let msg = Message::Request { methods: vec![] };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> =
        [("methods".to_string(), AttributeValue::L(vec![]))].into();

    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Message::Request {
        methods: vec!["GET".into(), "POST".into()],
    };

    let inner: HashMap<String, AttributeValue> = [(
        "methods".to_string(),
        AttributeValue::L(vec![
            AttributeValue::S("GET".into()),
            AttributeValue::S("POST".into()),
        ]),
    )]
    .into();

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
fn test_try_from_hashmap_into_empty_vector() {
    let expected = Message::Request { methods: vec![] };

    let inner: HashMap<String, AttributeValue> =
        [("methods".to_string(), AttributeValue::L(vec![]))].into();

    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_field_not_set() {
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(HashMap::new()))].into();

    let actual = Message::try_from(item);

    assert_field_not_set!(actual, "methods");
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
        [("methods".to_string(), AttributeValue::S("GET".into()))].into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_attribute_unmatch!(actual, "L");

    let inner: HashMap<String, AttributeValue> = [(
        "methods".to_string(),
        AttributeValue::L(vec![AttributeValue::N("10".into())]),
    )]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_attribute_unmatch!(actual, "S");
}
