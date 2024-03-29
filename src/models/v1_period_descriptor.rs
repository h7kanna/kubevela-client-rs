/*
 * Kubevela api doc
 *
 * Kubevela api doc
 *
 * The version of the OpenAPI document: 1.9.7
 * Contact: feedback@mail.kubevela.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1PeriodDescriptor {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "artifactType", skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "digest")]
    pub digest: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<crate::models::V1PeriodPlatform>>,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl V1PeriodDescriptor {
    pub fn new(digest: String, media_type: String, size: i64) -> V1PeriodDescriptor {
        V1PeriodDescriptor {
            annotations: None,
            artifact_type: None,
            data: None,
            digest,
            media_type,
            platform: None,
            size,
            urls: None,
        }
    }
}


