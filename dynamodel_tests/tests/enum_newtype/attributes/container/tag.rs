use super::*;

#[derive(Dynamodel, Debug, PartialEq)]
#[dynamodel(tag = "type")]
enum NewType {
    Val(Example),
}

#[derive(Dynamodel, Debug, PartialEq)]
#[dynamodel(tag = "type")]
enum NewTypeOpt {
    Val(Option<Example>),
}

#[derive(Dynamodel, Debug, PartialEq)]
struct Example {
    id: String,
}

#[test]
fn test_into_hashmap() {
    let val = NewType::Val(Example { id: "foo".into() });
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Val".into())),
        ("id".to_string(), AttributeValue::S("foo".into())),
    ]
    .into();

    assert_eq!(actual, expected);

    let val = NewTypeOpt::Val(Some(Example { id: "foo".into() }));
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Val".into())),
        ("id".to_string(), AttributeValue::S("foo".into())),
    ]
    .into();

    assert_eq!(actual, expected);

    let val = NewTypeOpt::Val(None);
    let actual: HashMap<String, AttributeValue> = val.into();

    let expected: HashMap<String, AttributeValue> =
        [("type".to_string(), AttributeValue::S("Val".into()))].into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = NewType::Val(Example { id: "foo".into() });

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Val".into())),
        ("id".to_string(), AttributeValue::S("foo".into())),
    ]
    .into();

    let actual = NewType::try_from(item);

    assert_ok_eq!(actual, expected);

    let expected = NewTypeOpt::Val(Some(Example { id: "foo".into() }));

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Val".into())),
        ("id".to_string(), AttributeValue::S("foo".into())),
    ]
    .into();

    let actual = NewTypeOpt::try_from(item);

    assert_ok_eq!(actual, expected);

    let expected = NewTypeOpt::Val(None);

    let item: HashMap<String, AttributeValue> =
        [("type".to_string(), AttributeValue::S("Val".into()))].into();

    let actual = NewTypeOpt::try_from(item);

    assert_ok_eq!(actual, expected);
}
