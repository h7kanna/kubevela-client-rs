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
pub struct V1PeriodCreatePolicyRequest {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "envName")]
    pub env_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "properties")]
    pub properties: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "workflowPolicyBind")]
    pub workflow_policy_bind: Vec<crate::models::V1PeriodWorkflowPolicyBinding>,
}

impl V1PeriodCreatePolicyRequest {
    pub fn new(alias: String, description: String, env_name: String, name: String, properties: String, r#type: String, workflow_policy_bind: Vec<crate::models::V1PeriodWorkflowPolicyBinding>) -> V1PeriodCreatePolicyRequest {
        V1PeriodCreatePolicyRequest {
            alias,
            description,
            env_name,
            name,
            properties,
            r#type,
            workflow_policy_bind,
        }
    }
}


