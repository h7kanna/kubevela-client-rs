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
pub struct V1PeriodUpdatePolicyRequest {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "envName")]
    pub env_name: String,
    #[serde(rename = "properties")]
    pub properties: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "workflowPolicyBind")]
    pub workflow_policy_bind: Vec<crate::models::V1PeriodWorkflowPolicyBinding>,
}

impl V1PeriodUpdatePolicyRequest {
    pub fn new(alias: String, description: String, env_name: String, properties: String, r#type: String, workflow_policy_bind: Vec<crate::models::V1PeriodWorkflowPolicyBinding>) -> V1PeriodUpdatePolicyRequest {
        V1PeriodUpdatePolicyRequest {
            alias,
            description,
            env_name,
            properties,
            r#type,
            workflow_policy_bind,
        }
    }
}


