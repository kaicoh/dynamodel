use super::*;

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum UpperCaseEx {
    #[dynamodel(rename_all = "UPPERCASE")]
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
fn test_uppercase_externally_tagged() {
    let m = UpperCaseEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("ID".to_string(), AttributeValue::S("1".into())),
        ("METHOD_NAME".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = UpperCaseEx::Response {
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

    let converted: UpperCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum UpperCaseIn {
    #[dynamodel(rename_all = "UPPERCASE")]
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
fn test_uppercase_internally_tagged() {
    let m = UpperCaseIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("ID".to_string(), AttributeValue::S("1".into())),
        ("METHOD_NAME".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = UpperCaseIn::Response {
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

    let converted: UpperCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum PascalCaseEx {
    #[dynamodel(rename_all = "PascalCase")]
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
fn test_pascalcase_externally_tagged() {
    let m = PascalCaseEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("Id".to_string(), AttributeValue::S("1".into())),
        ("MethodName".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: PascalCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = PascalCaseEx::Response {
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

    let converted: PascalCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum PascalCaseIn {
    #[dynamodel(rename_all = "PascalCase")]
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
fn test_pascalcase_internally_tagged() {
    let m = PascalCaseIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("Id".to_string(), AttributeValue::S("1".into())),
        ("MethodName".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: PascalCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = PascalCaseIn::Response {
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

    let converted: PascalCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum CamelCaseEx {
    #[dynamodel(rename_all = "camelCase")]
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
fn test_camelcase_externally_tagged() {
    let m = CamelCaseEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("methodName".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = CamelCaseEx::Response {
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

    let converted: CamelCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum CamelCaseIn {
    #[dynamodel(rename_all = "camelCase")]
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
fn test_camelcase_internally_tagged() {
    let m = CamelCaseIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("methodName".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = CamelCaseIn::Response {
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

    let converted: CamelCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum ScreamingSnakeCaseEx {
    #[dynamodel(rename_all = "SCREAMING_SNAKE_CASE")]
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
fn test_screamingsnakecase_externally_tagged() {
    let m = ScreamingSnakeCaseEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("ID".to_string(), AttributeValue::S("1".into())),
        ("METHOD_NAME".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = ScreamingSnakeCaseEx::Response {
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

    let converted: ScreamingSnakeCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum ScreamingSnakeCaseIn {
    #[dynamodel(rename_all = "SCREAMING_SNAKE_CASE")]
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
fn test_screamingsnakecase_internally_tagged() {
    let m = ScreamingSnakeCaseIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("ID".to_string(), AttributeValue::S("1".into())),
        ("METHOD_NAME".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = ScreamingSnakeCaseIn::Response {
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

    let converted: ScreamingSnakeCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum KebabCaseEx {
    #[dynamodel(rename_all = "kebab-case")]
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
fn test_kebabcase_externally_tagged() {
    let m = KebabCaseEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method-name".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = KebabCaseEx::Response {
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

    let converted: KebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum KebabCaseIn {
    #[dynamodel(rename_all = "kebab-case")]
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
fn test_kebabcase_internally_tagged() {
    let m = KebabCaseIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("id".to_string(), AttributeValue::S("1".into())),
        ("method-name".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = KebabCaseIn::Response {
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

    let converted: KebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum ScreamingKebabCaseEx {
    #[dynamodel(rename_all = "SCREAMING-KEBAB-CASE")]
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
fn test_screamingkebabcase_externally_tagged() {
    let m = ScreamingKebabCaseEx::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let inner: HashMap<String, AttributeValue> = [
        ("ID".to_string(), AttributeValue::S("1".into())),
        ("METHOD-NAME".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();
    let item: HashMap<String, AttributeValue> =
        [("Request".to_string(), AttributeValue::M(inner))].into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = ScreamingKebabCaseEx::Response {
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

    let converted: ScreamingKebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum ScreamingKebabCaseIn {
    #[dynamodel(rename_all = "SCREAMING-KEBAB-CASE")]
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
fn test_screamingkebabcase_internally_tagged() {
    let m = ScreamingKebabCaseIn::Request {
        id: "1".into(),
        method_name: "GET".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Request".into())),
        ("ID".to_string(), AttributeValue::S("1".into())),
        ("METHOD-NAME".to_string(), AttributeValue::S("GET".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);

    let m = ScreamingKebabCaseIn::Response {
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

    let converted: ScreamingKebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, m);
}
