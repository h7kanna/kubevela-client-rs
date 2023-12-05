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
pub struct V1PeriodImageInfo {
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<crate::models::V1PeriodConfigFile>>,
    #[serde(rename = "manifest")]
    pub manifest: Box<crate::models::V1PeriodManifest>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "registry")]
    pub registry: String,
    #[serde(rename = "secretNames")]
    pub secret_names: Vec<String>,
    #[serde(rename = "size")]
    pub size: i64,
}

impl V1PeriodImageInfo {
    pub fn new(manifest: crate::models::V1PeriodManifest, name: String, registry: String, secret_names: Vec<String>, size: i64) -> V1PeriodImageInfo {
        V1PeriodImageInfo {
            info: None,
            manifest: Box::new(manifest),
            message: None,
            name,
            registry,
            secret_names,
            size,
        }
    }
}


