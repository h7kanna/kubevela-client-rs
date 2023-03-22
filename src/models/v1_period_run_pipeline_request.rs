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
pub struct V1PeriodRunPipelineRequest {
    #[serde(rename = "contextName")]
    pub context_name: String,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<crate::models::V1alpha1PeriodWorkflowExecuteMode>>,
}

impl V1PeriodRunPipelineRequest {
    pub fn new(context_name: String) -> V1PeriodRunPipelineRequest {
        V1PeriodRunPipelineRequest {
            context_name,
            mode: None,
        }
    }
}

