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
pub struct ModelPeriodWorkflowStep {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<crate::models::V1alpha1PeriodInputItem>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::V1alpha1PeriodWorkflowStepMeta>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "orderIndex")]
    pub order_index: i32,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<crate::models::V1alpha1PeriodOutputItem>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "subSteps", skip_serializing_if = "Option::is_none")]
    pub sub_steps: Option<Vec<crate::models::ModelPeriodWorkflowStepBase>>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ModelPeriodWorkflowStep {
    pub fn new(alias: String, depends_on: Vec<String>, description: String, name: String, order_index: i32, r#type: String) -> ModelPeriodWorkflowStep {
        ModelPeriodWorkflowStep {
            alias,
            depends_on,
            description,
            r#if: None,
            inputs: None,
            meta: None,
            mode: None,
            name,
            order_index,
            outputs: None,
            properties: None,
            sub_steps: None,
            timeout: None,
            r#type,
        }
    }
}


