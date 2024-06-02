use super::*;

#[derive(Dynamodel, Debug, Clone, PartialEq)]
enum NewTypeEx {
    #[dynamodel(rename = "Renamed")]
    Val(Example),
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
#[dynamodel(tag = "type")]
enum NewTypeIn {
    #[dynamodel(rename = "Renamed")]
    Val(Example),
}

#[derive(Dynamodel, Debug, Clone, PartialEq)]
struct Example {
    id: String,
}

impl Example {
    fn new(id: &str) -> Self {
        Self { id: id.into() }
    }
}

#[test]
fn test_rename_variant_externally_tagged() {
    let val = NewTypeEx::Val(Example::new("foo"));
    let item: HashMap<String, AttributeValue> = [(
        "Renamed".to_string(),
        AttributeValue::M(Example::new("foo").into()),
    )]
    .into();

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: NewTypeEx = item.try_into().unwrap();
    assert_eq!(converted, val);
}

#[test]
fn test_rename_variant_internally_tagged() {
    let val = NewTypeIn::Val(Example::new("foo"));
    let mut item: HashMap<String, AttributeValue> = Example::new("foo").into();
    item.insert("type".into(), AttributeValue::S("Renamed".into()));

    let converted: HashMap<String, AttributeValue> = val.clone().into();
    assert_eq!(converted, item);

    let converted: NewTypeIn = item.try_into().unwrap();
    assert_eq!(converted, val);
}
