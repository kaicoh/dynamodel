use super::*;

#[derive(Dynamodel, Debug, PartialEq, Clone)]
struct Person {
    #[dynamodel(rename = "GivenName")]
    first_name: String,
    #[dynamodel(rename = "FamilyName")]
    last_name: String,
}

#[test]
fn test_rename_field() {
    let person = Person {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("GivenName".to_string(), AttributeValue::S("Kanji".into())),
        ("FamilyName".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = person.clone().into();
    assert_eq!(converted, item);

    let converted: Person = item.try_into().unwrap();
    assert_eq!(converted, person);
}
