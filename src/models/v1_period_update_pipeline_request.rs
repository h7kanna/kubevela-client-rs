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
pub struct V1PeriodUpdatePipelineRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::ModelPeriodWorkflowSpec>>,
}

impl V1PeriodUpdatePipelineRequest {
    pub fn new() -> V1PeriodUpdatePipelineRequest {
        V1PeriodUpdatePipelineRequest {
            alias: None,
            description: None,
            spec: None,
        }
    }
}

