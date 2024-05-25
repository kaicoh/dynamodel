use super::*;

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "lowercase")]
enum LowerCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_lowercase_externally_tagged() {
    let m = LowerCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: LowerCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "lowercase")]
enum LowerCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_lowercase_internally_tagged() {
    let m = LowerCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: LowerCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "UPPERCASE")]
enum UpperCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_uppercase_externally_tagged() {
    let m = UpperCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("REQUEST".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "UPPERCASE")]
enum UpperCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_uppercase_internally_tagged() {
    let m = UpperCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("REQUEST".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "camelCase")]
enum CamelCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_camelcase_externally_tagged() {
    let m = CamelCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "camelCase")]
enum CamelCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_camelcase_internally_tagged() {
    let m = CamelCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "snake_case")]
enum SnakeCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_snakecase_externally_tagged() {
    let m = SnakeCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: SnakeCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "snake_case")]
enum SnakeCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_snakecase_internally_tagged() {
    let m = SnakeCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: SnakeCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "SCREAMING_SNAKE_CASE")]
enum ScreamingSnakeCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_screamingsnakecase_externally_tagged() {
    let m = ScreamingSnakeCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("REQUEST".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum ScreamingSnakeCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_screamingsnakecase_internally_tagged() {
    let m = ScreamingSnakeCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("REQUEST".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "kebab-case")]
enum KebabCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_kebabcase_externally_tagged() {
    let m = KebabCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "kebab-case")]
enum KebabCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_kebabcase_internally_tagged() {
    let m = KebabCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "SCREAMING-KEBAB-CASE")]
enum ScreamingKebabCaseEx {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_screamingkebabcase_externally_tagged() {
    let m = ScreamingKebabCaseEx::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("REQUEST".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "SCREAMING-KEBAB-CASE")]
enum ScreamingKebabCaseIn {
    Request { id: String, method: String },
    Response { id: String, result: String },
}

#[test]
fn test_screamingkebabcase_internally_tagged() {
    let m = ScreamingKebabCaseIn::Request {
        id: "1".into(),
        method: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("REQUEST".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}
