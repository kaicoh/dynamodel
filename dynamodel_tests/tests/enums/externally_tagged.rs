use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum Message {
    #[allow(dead_code)]
    Request { id: String, method: String },
    #[allow(dead_code)]
    Response { id: String, result: String },
}

#[test]
fn test_into_hashmap() {
    let msg = Message::Request {
        id: "1".into(),
        method: "GET".into(),
    };
    let actual: HashMap<String, AttributeValue> = msg.into();

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let expected: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Message::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let actual = Message::try_from(item);

    assert_ok_eq!(actual, expected);
}
