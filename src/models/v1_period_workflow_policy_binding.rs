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
pub struct V1PeriodWorkflowPolicyBinding {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "steps")]
    pub steps: Vec<String>,
}

impl V1PeriodWorkflowPolicyBinding {
    pub fn new(name: String, steps: Vec<String>) -> V1PeriodWorkflowPolicyBinding {
        V1PeriodWorkflowPolicyBinding {
            name,
            steps,
        }
    }
}


