use super::*;

macro_rules! int_test {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                #[derive(Dynamodel, Debug, PartialEq)]
                enum [<Message$ty>] {
                    Request { ids: Vec<$ty> },
                    Response { id: String, result: String },
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let msg = [<Message$ty>]::Request {
                        ids: vec![10, 20],
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![
                                AttributeValue::N("10".into()),
                                AttributeValue::N("20".into()),
                            ])
                        ),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _into_hashmap_from_empty_vector>]() {
                    let msg = [<Message$ty>]::Request {
                        ids: vec![],
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        ("ids".to_string(), AttributeValue::L(vec![])),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<Message$ty>]::Request {
                        ids: vec![10, 20],
                    };

                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![
                                AttributeValue::N("10".into()),
                                AttributeValue::N("20".into()),
                            ])
                        ),
                    ]
                    .into();

                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_into_empty_vector>]() {
                    let expected = [<Message$ty>]::Request {
                        ids: vec![],
                    };

                    let inner: HashMap<String, AttributeValue> =
                        [("ids".to_string(), AttributeValue::L(vec![]))].into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_variant_not_found>]() {
                    let item: HashMap<String, AttributeValue> = HashMap::new();
                    let actual = [<Message$ty>]::try_from(item);

                    assert_variant_not_found!(actual);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_field_not_set>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(HashMap::new()))].into();
                    let actual = [<Message$ty>]::try_from(item);

                    assert_field_not_set!(actual, "ids");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::S("foo".into()))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "M");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_inner_unmatched_attribute_value>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        ("ids".to_string(), AttributeValue::S("10".into())),
                    ]
                    .into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "L");

                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![AttributeValue::S("10".into())]),
                        ),
                    ].into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![AttributeValue::N("foo".into())])
                        ),
                    ]
                    .into();

                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_parse_int!(actual);
                }
            }
         )*
    }
}

int_test!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! float_test {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                #[derive(Dynamodel, Debug, PartialEq)]
                enum [<Message$ty>] {
                    Request { ids: Vec<$ty> },
                    Response { id: String, result: String },
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let msg = [<Message$ty>]::Request {
                        ids: vec![1.2, 3.45],
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![
                                AttributeValue::N("1.2".into()),
                                AttributeValue::N("3.45".into()),
                            ])
                        ),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _into_hashmap_from_empty_vector>]() {
                    let msg = [<Message$ty>]::Request {
                        ids: vec![],
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        ("ids".to_string(), AttributeValue::L(vec![])),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<Message$ty>]::Request {
                        ids: vec![1.2, 3.45],
                    };

                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![
                                AttributeValue::N("1.2".into()),
                                AttributeValue::N("3.45".into()),
                            ])
                        ),
                    ]
                    .into();

                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_into_empty_vector>]() {
                    let expected = [<Message$ty>]::Request {
                        ids: vec![],
                    };

                    let inner: HashMap<String, AttributeValue> =
                        [("ids".to_string(), AttributeValue::L(vec![]))].into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_variant_not_found>]() {
                    let item: HashMap<String, AttributeValue> = HashMap::new();
                    let actual = [<Message$ty>]::try_from(item);

                    assert_variant_not_found!(actual);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_field_not_set>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(HashMap::new()))].into();
                    let actual = [<Message$ty>]::try_from(item);

                    assert_field_not_set!(actual, "ids");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::S("foo".into()))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "M");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_inner_unmatched_attribute_value>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        ("ids".to_string(), AttributeValue::S("1.2".into())),
                    ]
                    .into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "L");

                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![AttributeValue::S("1.2".into())]),
                        ),
                    ].into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        (
                            "ids".to_string(),
                            AttributeValue::L(vec![AttributeValue::N("foo".into())])
                        ),
                    ]
                    .into();

                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_parse_float!(actual);
                }
            }
         )*
    }
}

float_test!(f32, f64);
