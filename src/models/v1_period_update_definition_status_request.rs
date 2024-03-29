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
pub struct V1PeriodUpdateDefinitionStatusRequest {
    #[serde(rename = "hiddenInUI")]
    pub hidden_in_ui: bool,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl V1PeriodUpdateDefinitionStatusRequest {
    pub fn new(hidden_in_ui: bool, r#type: String) -> V1PeriodUpdateDefinitionStatusRequest {
        V1PeriodUpdateDefinitionStatusRequest {
            hidden_in_ui,
            r#type,
        }
    }
}


