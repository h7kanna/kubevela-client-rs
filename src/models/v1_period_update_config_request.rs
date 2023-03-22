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
pub struct V1PeriodUpdateConfigRequest {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

impl V1PeriodUpdateConfigRequest {
    pub fn new(alias: String, description: String) -> V1PeriodUpdateConfigRequest {
        V1PeriodUpdateConfigRequest {
            alias,
            description,
            properties: None,
        }
    }
}


