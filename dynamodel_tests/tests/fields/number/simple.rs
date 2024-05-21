use super::*;

macro_rules! int_test {
    ($($ty:ty),*) => {
        $(
            paste::item! {
                #[derive(Debug, Dynamodel, PartialEq)]
                struct [<Model$ty>] {
                    attr: $ty,
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let m = [<Model$ty>] { attr: 10 };
                    let actual: HashMap<String, AttributeValue> = m.into();

                    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
                    expected.insert("attr".into(), AttributeValue::N("10".into()));

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<Model$ty>] { attr: 10 };

                    let mut item: HashMap<String, AttributeValue> = HashMap::new();
                    item.insert("attr".into(), AttributeValue::N("10".into()));
                    let actual = [<Model$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_field_not_set>]() {
                    let item: HashMap<String, AttributeValue> = HashMap::new();
                    let actual = [<Model$ty>]::try_from(item);

                    assert_field_not_set!(actual, "attr");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let mut item: HashMap<String, AttributeValue> = HashMap::new();
                    item.insert("attr".into(), AttributeValue::S("10".into()));
                    let actual = [<Model$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let mut item: HashMap<String, AttributeValue> = HashMap::new();
                    item.insert("attr".into(), AttributeValue::N("foo".into()));
                    let actual = [<Model$ty>]::try_from(item);

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
                #[derive(Debug, Dynamodel, PartialEq)]
                struct [<Model$ty>] {
                    attr: $ty,
                }

                #[test]
                fn [<test_ $ty _into_hashmap>]() {
                    let m = [<Model$ty>] { attr: 1.2 };
                    let actual: HashMap<String, AttributeValue> = m.into();

                    let mut expected: HashMap<String, AttributeValue> = HashMap::new();
                    expected.insert("attr".into(), AttributeValue::N("1.2".into()));

                    assert_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap>]() {
                    let expected = [<Model$ty>] { attr: 1.2 };

                    let mut item: HashMap<String, AttributeValue> = HashMap::new();
                    item.insert("attr".into(), AttributeValue::N("1.2".into()));
                    let actual = [<Model$ty>]::try_from(item);

                    assert_ok_eq!(actual, expected);
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_field_not_set>]() {
                    let item: HashMap<String, AttributeValue> = HashMap::new();
                    let actual = [<Model$ty>]::try_from(item);

                    assert_field_not_set!(actual, "attr");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_unmatched_attribute_value>]() {
                    let mut item: HashMap<String, AttributeValue> = HashMap::new();
                    item.insert("attr".into(), AttributeValue::S("1.2".into()));
                    let actual = [<Model$ty>]::try_from(item);

                    assert_attribute_unmatch!(actual, "N");
                }

                #[test]
                fn [<test_ $ty _try_from_hashmap_parse_err>]() {
                    let mut item: HashMap<String, AttributeValue> = HashMap::new();
                    item.insert("attr".into(), AttributeValue::N("foo".into()));
                    let actual = [<Model$ty>]::try_from(item);

                    assert_parse_float!(actual);
                }
            }
         )*
    }
}

float_test!(f32, f64);
