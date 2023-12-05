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
pub struct V1alpha1PeriodWorkflowStepMeta {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

impl V1alpha1PeriodWorkflowStepMeta {
    pub fn new() -> V1alpha1PeriodWorkflowStepMeta {
        V1alpha1PeriodWorkflowStepMeta {
            alias: None,
        }
    }
}


