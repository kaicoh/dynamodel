use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
enum NewType {
    Var(Vec<Inner>),
}

#[derive(Dynamodel, Debug, PartialEq)]
struct Inner {
    attr: String,
}

#[test]
fn test_into_hashmap() {
    let val = NewType::Var(vec![
        Inner { attr: "foo".into() },
        Inner { attr: "bar".into() },
    ]);
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> = [(
        "Var".to_string(),
        AttributeValue::L(vec![
            AttributeValue::M(Inner { attr: "foo".into() }.into()),
            AttributeValue::M(Inner { attr: "bar".into() }.into()),
        ]),
    )]
    .into();

    assert_eq!(actual, expected);
}

#[test]
fn test_into_hashmap_from_empty_vector() {
    let val = NewType::Var(vec![]);
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> =
        [("Var".to_string(), AttributeValue::L(vec![]))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = NewType::Var(vec![
        Inner { attr: "foo".into() },
        Inner { attr: "bar".into() },
    ]);

    let item: HashMap<String, AttributeValue> = [(
        "Var".to_string(),
        AttributeValue::L(vec![
            AttributeValue::M(Inner { attr: "foo".into() }.into()),
            AttributeValue::M(Inner { attr: "bar".into() }.into()),
        ]),
    )]
    .into();
    let actual = NewType::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_from_empty_vector() {
    let expected = NewType::Var(vec![]);

    let item: HashMap<String, AttributeValue> =
        [("Var".to_string(), AttributeValue::L(vec![]))].into();
    let actual = NewType::try_from(item);

    assert_ok_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap_variant_not_found() {
    let item: HashMap<String, AttributeValue> = HashMap::new();
    let actual = NewType::try_from(item);

    assert_variant_not_found!(actual);
}

#[test]
fn test_try_from_hashmap_unmatched_attribute_value() {
    let item: HashMap<String, AttributeValue> =
        [("Var".to_string(), AttributeValue::S("foo".into()))].into();

    let actual = NewType::try_from(item);

    assert_attribute_unmatch!(actual, "L");

    let item: HashMap<String, AttributeValue> = [(
        "Var".to_string(),
        AttributeValue::L(vec![AttributeValue::S("foo".into())]),
    )]
    .into();
    let actual = NewType::try_from(item);

    assert_attribute_unmatch!(actual, "M");
}
