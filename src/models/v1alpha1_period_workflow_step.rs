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
pub struct V1alpha1PeriodWorkflowStep {
    #[serde(rename = "dependsOn", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<crate::models::V1alpha1PeriodInputItem>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::V1alpha1PeriodWorkflowStepMeta>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<crate::models::V1alpha1PeriodOutputItem>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "subSteps", skip_serializing_if = "Option::is_none")]
    pub sub_steps: Option<Vec<crate::models::V1alpha1PeriodWorkflowStepBase>>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl V1alpha1PeriodWorkflowStep {
    pub fn new(r#type: String) -> V1alpha1PeriodWorkflowStep {
        V1alpha1PeriodWorkflowStep {
            depends_on: None,
            r#if: None,
            inputs: None,
            meta: None,
            name: None,
            outputs: None,
            properties: None,
            sub_steps: None,
            timeout: None,
            r#type,
        }
    }
}

