use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
#[dynamodel(tag = "type")]
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

    let expected: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = Message::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let actual = Message::try_from(item);

    assert_ok_eq!(actual, expected);
}
