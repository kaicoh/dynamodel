use aws_sdk_dynamodb::types::AttributeValue;
use dynamodel::Dynamodel;
use std::collections::HashMap;

#[derive(Dynamodel, Debug, PartialEq, Clone)]
#[dynamodel(extra = "Video::key", rename_all = "PascalCase")]
struct Video {
    #[dynamodel(rename = "PK")]
    id: String,
    author: String,
    uploaded_at: String,
}

impl Video {
    fn key(&self) -> HashMap<String, AttributeValue> {
        [("SK".to_string(), AttributeValue::S("Video".into()))].into()
    }
}

#[test]
fn test_convert_with_extra() {
    let v = Video {
        id: "6b8c736e".into(),
        author: "VideoGal12".into(),
        uploaded_at: "2022-07-06T13:41:28".into(),
    };

    let item: HashMap<String, AttributeValue> = [
        ("PK".to_string(), AttributeValue::S("6b8c736e".into())),
        ("SK".to_string(), AttributeValue::S("Video".into())),
        ("Author".to_string(), AttributeValue::S("VideoGal12".into())),
        (
            "UploadedAt".to_string(),
            AttributeValue::S("2022-07-06T13:41:28".into()),
        ),
    ]
    .into();

    let converted: HashMap<String, AttributeValue> = v.clone().into();
    assert_eq!(converted, item);

    let item: HashMap<String, AttributeValue> = [
        ("PK".to_string(), AttributeValue::S("6b8c736e".into())),
        ("SK".to_string(), AttributeValue::S("Video".into())),
        ("Author".to_string(), AttributeValue::S("VideoGal12".into())),
        (
            "UploadedAt".to_string(),
            AttributeValue::S("2022-07-06T13:41:28".into()),
        ),
    ]
    .into();

    let converted: Video = item.try_into().unwrap();
    assert_eq!(converted, v);
}
