use super::*;

#[derive(Dynamodel, Debug, Clone, PartialEq)]
struct Example {
    id: String,
}

impl Example {
    fn new(id: &str) -> Self {
        Self { id: id.into() }
    }
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "lowercase")]
enum LowerCaseEx {
    NewType(Example),
}

#[test]
fn test_lowercase_externally_tagged() {
    let val = LowerCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "newtype".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: LowerCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "lowercase")]
enum LowerCaseIn {
    NewType(Example),
}

#[test]
fn test_lowercase_internally_tagged() {
    let val = LowerCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("newtype".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: LowerCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "UPPERCASE")]
enum UpperCaseEx {
    NewType(Example),
}

#[test]
fn test_uppercase_externally_tagged() {
    let val = UpperCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "NEWTYPE".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "UPPERCASE")]
enum UpperCaseIn {
    NewType(Example),
}

#[test]
fn test_uppercase_internally_tagged() {
    let val = UpperCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("NEWTYPE".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "camelCase")]
enum CamelCaseEx {
    NewType(Example),
}

#[test]
fn test_camelcase_externally_tagged() {
    let val = CamelCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "newType".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "camelCase")]
enum CamelCaseIn {
    NewType(Example),
}

#[test]
fn test_camelcase_internally_tagged() {
    let val = CamelCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("newType".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "snake_case")]
enum SnakeCaseEx {
    NewType(Example),
}

#[test]
fn test_snakecase_externally_tagged() {
    let val = SnakeCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "new_type".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: SnakeCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "snake_case")]
enum SnakeCaseIn {
    NewType(Example),
}

#[test]
fn test_snakecase_internally_tagged() {
    let val = SnakeCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("new_type".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: SnakeCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "SCREAMING_SNAKE_CASE")]
enum ScreamingSnakeCaseEx {
    NewType(Example),
}

#[test]
fn test_screamingsnakecase_externally_tagged() {
    let val = ScreamingSnakeCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "NEW_TYPE".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum ScreamingSnakeCaseIn {
    NewType(Example),
}

#[test]
fn test_screamingsnakecase_internally_tagged() {
    let val = ScreamingSnakeCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("NEW_TYPE".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "kebab-case")]
enum KebabCaseEx {
    NewType(Example),
}

#[test]
fn test_kebabcase_externally_tagged() {
    let val = KebabCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "new-type".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "kebab-case")]
enum KebabCaseIn {
    NewType(Example),
}

#[test]
fn test_kebabcase_internally_tagged() {
    let val = KebabCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("new-type".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(rename_all = "SCREAMING-KEBAB-CASE")]
enum ScreamingKebabCaseEx {
    NewType(Example),
}

#[test]
fn test_screamingkebabcase_externally_tagged() {
    let val = ScreamingKebabCaseEx::NewType(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "NEW-TYPE".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCaseEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type", rename_all = "SCREAMING-KEBAB-CASE")]
enum ScreamingKebabCaseIn {
    NewType(Example),
}

#[test]
fn test_screamingkebabcase_internally_tagged() {
    let val = ScreamingKebabCaseIn::NewType(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("NEW-TYPE".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCaseIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}
