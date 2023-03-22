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
pub struct V1beta1PeriodWorkflow {
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<crate::models::V1alpha1PeriodWorkflowExecuteMode>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<crate::models::V1alpha1PeriodWorkflowStep>>,
}

impl V1beta1PeriodWorkflow {
    pub fn new() -> V1beta1PeriodWorkflow {
        V1beta1PeriodWorkflow {
            mode: None,
            r#ref: None,
            steps: None,
        }
    }
}


