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
pub struct V1PeriodPluginEnableRequest {
    #[serde(rename = "jsonData")]
    pub json_data: serde_json::Value,
    #[serde(rename = "secureJsonData")]
    pub secure_json_data: serde_json::Value,
}

impl V1PeriodPluginEnableRequest {
    pub fn new(json_data: serde_json::Value, secure_json_data: serde_json::Value) -> V1PeriodPluginEnableRequest {
        V1PeriodPluginEnableRequest {
            json_data,
            secure_json_data,
        }
    }
}

