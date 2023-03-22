/*
 * Kubevela api doc
 *
 * Kubevela api doc
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: feedback@mail.kubevela.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1PeriodPlatform {
    #[serde(rename = "architecture")]
    pub architecture: String,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "os")]
    pub os: String,
    #[serde(rename = "os.features", skip_serializing_if = "Option::is_none")]
    pub os_period_features: Option<Vec<String>>,
    #[serde(rename = "os.version", skip_serializing_if = "Option::is_none")]
    pub os_period_version: Option<String>,
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl V1PeriodPlatform {
    pub fn new(architecture: String, os: String) -> V1PeriodPlatform {
        V1PeriodPlatform {
            architecture,
            features: None,
            os,
            os_period_features: None,
            os_period_version: None,
            variant: None,
        }
    }
}


