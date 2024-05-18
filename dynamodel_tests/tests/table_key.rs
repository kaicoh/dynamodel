use aws_sdk_dynamodb::types::AttributeValue;
use dynamodel::Dynamodel;
use std::collections::HashMap;

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(table_key = "Video::key")]
struct Video {
    id: String,
    author: String,
    uploaded_at: String,
}

impl Video {
    fn key(&self) -> HashMap<String, AttributeValue> {
        [
            ("PK".to_string(), AttributeValue::S(self.id.clone())),
            ("SK".to_string(), AttributeValue::S("Video".into())),
        ]
        .into()
    }
}

#[test]
fn test_convert_with_table_key() {
    let v = Video {
        id: "6b8c736e".into(),
        author: "VideoGal12".into(),
        uploaded_at: "2022-07-06T13:41:28".into(),
    };

    let item_with_key: HashMap<String, AttributeValue> = [
        ("PK".to_string(), AttributeValue::S("6b8c736e".into())),
        ("SK".to_string(), AttributeValue::S("Video".into())),
        ("id".to_string(), AttributeValue::S("6b8c736e".into())),
        ("author".to_string(), AttributeValue::S("VideoGal12".into())),
        (
            "uploaded_at".to_string(),
            AttributeValue::S("2022-07-06T13:41:28".into()),
        ),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = v.clone().into();
    assert_eq!(converted, item_with_key);

    let item_without_key: HashMap<String, AttributeValue> = [
        ("id".to_string(), AttributeValue::S("6b8c736e".into())),
        ("author".to_string(), AttributeValue::S("VideoGal12".into())),
        (
            "uploaded_at".to_string(),
            AttributeValue::S("2022-07-06T13:41:28".into()),
        ),
    ]
    .into();

    let converted: Video = item_without_key.try_into().unwrap();
    assert_eq!(converted, v);
}
