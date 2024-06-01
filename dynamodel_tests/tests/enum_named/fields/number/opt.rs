use super::*;

macro_rules! int_test {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                #[derive(Dynamodel, Debug, PartialEq)]
                enum [<Message$ty>] {
                    Request { id: Option<$ty>, method: String },
                    Response { id: Option<$ty>, result: String },
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let msg = [<Message$ty>]::Request {
                        id: Some(10),
                        method: "GET".into(),
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::N("10".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _into_hashmap_from_none>]() {
                    let msg = [<Message$ty>]::Request {
                        id: None,
                        method: "GET".into(),
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<Message$ty>]::Request {
                        id: Some(10),
                        method: "GET".into(),
                    };

                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::N("10".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();

                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_into_none>]() {
                    let expected = [<Message$ty>]::Request {
                        id: None,
                        method: "GET".into(),
                    };

                    let inner: HashMap<String, AttributeValue> =
                        [("method".to_string(), AttributeValue::S("GET".into()))].into();
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
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::S("foo".into()))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "M");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_inner_unmatched_attribute_value>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::S("10".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::N("foo".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
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
                    Request { id: Option<$ty>, method: String },
                    Response { id: Option<$ty>, result: String },
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let msg = [<Message$ty>]::Request {
                        id: Some(1.2),
                        method: "GET".into(),
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::N("1.2".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _into_hashmap_from_none>]() {
                    let msg = [<Message$ty>]::Request {
                        id: None,
                        method: "GET".into(),
                    };
                    let actual: HashMap<String, AttributeValue> = msg.into();

                    let inner: HashMap<String, AttributeValue> = [
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();

                    let expected: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<Message$ty>]::Request {
                        id: Some(1.2),
                        method: "GET".into(),
                    };

                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::N("1.2".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();

                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_into_none>]() {
                    let expected = [<Message$ty>]::Request {
                        id: None,
                        method: "GET".into(),
                    };

                    let inner: HashMap<String, AttributeValue> =
                        [("method".to_string(), AttributeValue::S("GET".into()))].into();
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
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::S("foo".into()))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "M");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_inner_unmatched_attribute_value>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::S("1.2".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
                    ]
                    .into();
                    let item: HashMap<String, AttributeValue> =
                        [("Request".to_string(), AttributeValue::M(inner))].into();

                    let actual = [<Message$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let inner: HashMap<String, AttributeValue> = [
                        ("id".to_string(), AttributeValue::N("foo".into())),
                        ("method".to_string(), AttributeValue::S("GET".into())),
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
