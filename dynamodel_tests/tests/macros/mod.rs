macro_rules! assert_ok_eq {
    ($result:expr, $expected_inner:expr $(,)?) => {
        match $result {
            Ok(inner) => assert_eq!(inner, $expected_inner),
            Err(_) => unreachable!("{} should be an Ok", stringify!($result)),
        }
    };
}

macro_rules! assert_attribute_unmatch {
    ($result:expr, $expect_type:tt $(,)?) => {
        match $result {
            Err(err) => assert!(matches!(err, ConvertError::AttributeValueUnmatched(t, _) if t.as_str() == $expect_type)),
            _ => unreachable!("{} should be an ConvertError::AttributeValueUnmatched", stringify!($result)),
        }
    };
}

macro_rules! assert_field_not_set {
    ($result:expr, $expect_attr:tt $(,)?) => {
        match $result {
            Err(err) => assert!(matches!(err, ConvertError::FieldNotSet(t) if t.as_str() == $expect_attr)),
            _ => unreachable!("{} should be an ConvertError::FieldNotSet", stringify!($result)),
        }
    };
}

macro_rules! assert_parse_int {
    ($result:expr $(,)?) => {
        match $result {
            Err(err) => assert!(matches!(err, ConvertError::ParseInt(_))),
            _ => unreachable!(
                "{} should be an ConvertError::ParseInt",
                stringify!($result)
            ),
        }
    };
}

macro_rules! assert_parse_float {
    ($result:expr $(,)?) => {
        match $result {
            Err(err) => assert!(matches!(err, ConvertError::ParseFloat(_))),
            _ => unreachable!(
                "{} should be an ConvertError::ParseFloat",
                stringify!($result)
            ),
        }
    };
}

#[allow(unused_macros)]
macro_rules! assert_variant_not_found {
    ($result:expr $(,)?) => {
        match $result {
            Err(err) => assert!(matches!(err, ConvertError::VariantNotFound)),
            _ => unreachable!(
                "{} should be an ConvertError::VariantNotFound",
                stringify!($result)
            ),
        }
    };
}
