use super::*;

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(rename_all = "UPPERCASE")]
struct UpperCase {
    first_name: String,
    last_name: String,
}

#[test]
fn test_uppercase() {
    let m = UpperCase {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("FIRST_NAME".to_string(), AttributeValue::S("Kanji".into())),
        ("LAST_NAME".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: UpperCase = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(rename_all = "PascalCase")]
struct PascalCase {
    first_name: String,
    last_name: String,
}

#[test]
fn test_pascalcase() {
    let m = PascalCase {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("FirstName".to_string(), AttributeValue::S("Kanji".into())),
        ("LastName".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: PascalCase = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(rename_all = "camelCase")]
struct CamelCase {
    first_name: String,
    last_name: String,
}

#[test]
fn test_camelcase() {
    let m = CamelCase {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("firstName".to_string(), AttributeValue::S("Kanji".into())),
        ("lastName".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: CamelCase = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingSnakeCase {
    first_name: String,
    last_name: String,
}

#[test]
fn test_screamingsnakecase() {
    let m = ScreamingSnakeCase {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("FIRST_NAME".to_string(), AttributeValue::S("Kanji".into())),
        ("LAST_NAME".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingSnakeCase = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(rename_all = "kebab-case")]
struct KebabCase {
    first_name: String,
    last_name: String,
}

#[test]
fn test_kebabcase() {
    let m = KebabCase {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("first-name".to_string(), AttributeValue::S("Kanji".into())),
        ("last-name".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: KebabCase = item.try_into().unwrap();
    assert_eq!(converted, m);
}

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(rename_all = "SCREAMING-KEBAB-CASE")]
struct ScreamingKebabCase {
    first_name: String,
    last_name: String,
}

#[test]
fn test_screamingkababcase() {
    let m = ScreamingKebabCase {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("FIRST-NAME".to_string(), AttributeValue::S("Kanji".into())),
        ("LAST-NAME".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = m.clone().into();
    assert_eq!(converted, item);

    let converted: ScreamingKebabCase = item.try_into().unwrap();
    assert_eq!(converted, m);
}
