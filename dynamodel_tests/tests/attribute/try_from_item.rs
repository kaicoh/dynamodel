use super::*;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

#[derive(Debug, Dynamodel, PartialEq)]
#[dynamodel(extra = "VideoComment::sort_key")]
struct VideoComment {
    #[dynamodel(rename = "PK")]
    id: String,
    content: String,
    #[dynamodel(skip_into, try_from_item = "get_timestamp")]
    timestamp: String,
}

impl VideoComment {
    fn sort_key(&self) -> HashMap<String, AttributeValue> {
        [(
            "SK".to_string(),
            AttributeValue::S(format!("VideoComment#{}", self.timestamp)),
        )]
        .into()
    }
}

fn get_timestamp(item: &HashMap<String, AttributeValue>) -> Result<String, ConvertError> {
    item.get("SK")
        .ok_or(ConvertError::FieldNotSet("SK".into()))
        .and_then(|v| {
            v.as_s()
                .map_err(|e| ConvertError::AttributeValueUnmatched("S".into(), e.clone()))
        })
        .map(|v| v.split('#').last().unwrap().to_string())
}

#[test]
fn test_into_hashmap() {
    let comment = VideoComment {
        id: "12345".into(),
        content: "Good video".into(),
        timestamp: "2023-04-05T12:34:56".into(),
    };
    let actual: HashMap<String, AttributeValue> = comment.into();

    let expected: HashMap<String, AttributeValue> = [
        ("PK".to_string(), AttributeValue::S("12345".into())),
        (
            "SK".to_string(),
            AttributeValue::S("VideoComment#2023-04-05T12:34:56".into()),
        ),
        (
            "content".to_string(),
            AttributeValue::S("Good video".into()),
        ),
    ]
    .into();

    assert_eq!(actual, expected);
}

#[test]
fn test_try_from_hashmap() {
    let expected = VideoComment {
        id: "12345".into(),
        content: "Good video".into(),
        timestamp: "2023-04-05T12:34:56".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("PK".to_string(), AttributeValue::S("12345".into())),
        (
            "SK".to_string(),
            AttributeValue::S("VideoComment#2023-04-05T12:34:56".into()),
        ),
        (
            "content".to_string(),
            AttributeValue::S("Good video".into()),
        ),
    ]
    .into();
    let actual = VideoComment::try_from(item);

    assert_ok_eq!(actual, expected);
}
