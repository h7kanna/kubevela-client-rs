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
pub struct V1PeriodEnableAddonRequest {
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
    #[serde(rename = "clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    #[serde(rename = "registryName", skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl V1PeriodEnableAddonRequest {
    pub fn new() -> V1PeriodEnableAddonRequest {
        V1PeriodEnableAddonRequest {
            args: None,
            clusters: None,
            registry_name: None,
            version: None,
        }
    }
}


