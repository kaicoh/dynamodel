use super::*;

macro_rules!  int_test {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                #[derive(Dynamodel, Debug, PartialEq)]
                enum [<NewType$ty>] {
                    Nums(Vec<$ty>),
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let val = [<NewType$ty>]::Nums(vec![10, 15]);
                    let actual: HashMap<String, AttributeValue> = val.into();

                    let expected: HashMap<String, AttributeValue> = [
                        ("Nums".to_string(),
                        AttributeValue::L(vec![
                            AttributeValue::N("10".into()),
                            AttributeValue::N("15".into()),
                        ])),
                    ]
                    .into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _into_hashmap_from_empty_vector>]() {
                    let val = [<NewType$ty>]::Nums(vec![]);
                    let actual: HashMap<String, AttributeValue> = val.into();

                    let expected: HashMap<String, AttributeValue> = [(
                        "Nums".to_string(),
                        AttributeValue::L(vec![]),
                    )]
                    .into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<NewType$ty>]::Nums(vec![10, 15]);

                    let item: HashMap<String, AttributeValue> = [
                        ("Nums".to_string(),
                        AttributeValue::L(vec![
                            AttributeValue::N("10".into()),
                            AttributeValue::N("15".into()),
                        ])),
                    ]
                    .into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_variant_not_found>]() {
                    let item: HashMap<String, AttributeValue> = HashMap::new();
                    let actual = [<NewType$ty>]::try_from(item);

                    assert_variant_not_found!(actual);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Nums".to_string(), AttributeValue::S("foo".into()))].into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "L");

                    let item: HashMap<String, AttributeValue> =
                        [("Nums".to_string(), AttributeValue::L(vec![AttributeValue::S("foo".into())]))].into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Nums".to_string(), AttributeValue::L(vec![AttributeValue::N("foo".into())]))].into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_parse_int!(actual);
                }
            }
        )*
    }
}

int_test!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules!  float_test {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                #[derive(Dynamodel, Debug, PartialEq)]
                enum [<NewType$ty>] {
                    Nums(Vec<$ty>),
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let val = [<NewType$ty>]::Nums(vec![1.2, 3.45]);
                    let actual: HashMap<String, AttributeValue> = val.into();

                    let expected: HashMap<String, AttributeValue> = [
                        ("Nums".to_string(),
                        AttributeValue::L(vec![
                            AttributeValue::N("1.2".into()),
                            AttributeValue::N("3.45".into()),
                        ])),
                    ]
                    .into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _into_hashmap_from_empty_vector>]() {
                    let val = [<NewType$ty>]::Nums(vec![]);
                    let actual: HashMap<String, AttributeValue> = val.into();

                    let expected: HashMap<String, AttributeValue> = [(
                        "Nums".to_string(),
                        AttributeValue::L(vec![]),
                    )]
                    .into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<NewType$ty>]::Nums(vec![1.2, 3.45]);

                    let item: HashMap<String, AttributeValue> = [
                        ("Nums".to_string(),
                        AttributeValue::L(vec![
                            AttributeValue::N("1.2".into()),
                            AttributeValue::N("3.45".into()),
                        ])),
                    ]
                    .into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_variant_not_found>]() {
                    let item: HashMap<String, AttributeValue> = HashMap::new();
                    let actual = [<NewType$ty>]::try_from(item);

                    assert_variant_not_found!(actual);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Nums".to_string(), AttributeValue::S("foo".into()))].into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "L");

                    let item: HashMap<String, AttributeValue> =
                        [("Nums".to_string(), AttributeValue::L(vec![AttributeValue::S("foo".into())]))].into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Nums".to_string(), AttributeValue::L(vec![AttributeValue::N("foo".into())]))].into();

                    let actual = [<NewType$ty>]::try_from(item);

                    assert_parse_float!(actual);
                }
            }
        )*
    }
}

float_test!(f32, f64);
