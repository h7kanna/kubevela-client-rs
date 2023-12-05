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
pub struct RepoPeriodChartVersion {
    #[serde(rename = "Metadata")]
    pub metadata: Box<crate::models::ChartPeriodMetadata>,
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "engine", skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
    #[serde(rename = "tillerVersion", skip_serializing_if = "Option::is_none")]
    pub tiller_version: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "urls")]
    pub urls: Vec<String>,
}

impl RepoPeriodChartVersion {
    pub fn new(metadata: crate::models::ChartPeriodMetadata, urls: Vec<String>) -> RepoPeriodChartVersion {
        RepoPeriodChartVersion {
            metadata: Box::new(metadata),
            checksum: None,
            created: None,
            digest: None,
            engine: None,
            removed: None,
            tiller_version: None,
            url: None,
            urls,
        }
    }
}


