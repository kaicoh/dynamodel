use super::*;

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum MessageEx {
    #[dynamodel(rename = "Req")]
    Request {
        id: String,
        method_name: String,
    },
    Response {
        id: String,
        status_code: u16,
    },
}

#[test]
fn test_rename_variant_externally_tagged() {
    let m = MessageEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method_name".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Req".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: MessageEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = MessageEx::Response {
        id: "1".into(),
        status_code: 200,
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("status_code".to_string(), AttributeValue::N("200".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Response".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: MessageEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum MessageIn {
    #[dynamodel(rename = "Req")]
    Request {
        id: String,
        method_name: String,
    },
    Response {
        id: String,
        status_code: u16,
    },
}

#[test]
fn test_rename_variant_internally_tagged() {
    let m = MessageIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Req".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method_name".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: MessageIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = MessageIn::Response {
        id: "1".into(),
        status_code: 200,
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Response".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("status_code".to_string(), AttributeValue::N("200".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: MessageIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}
