use super::*;

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(tag = "type")]
struct Person {
    first_name: String,
    last_name: String,
}

#[test]
fn test_tagged_struct() {
    let person = Person {
        first_name: "Kanji".into(),
        last_name: "Tanaka".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("type".to_string(), AttributeValue::S("Person".into())),
        ("first_name".to_string(), AttributeValue::S("Kanji".into())),
        ("last_name".to_string(), AttributeValue::S("Tanaka".into())),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = person.clone().into();
    assert_eq!(converted, item);

    let converted: Person = item.try_into().unwrap();
    assert_eq!(converted, person);
}
