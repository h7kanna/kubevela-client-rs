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
pub struct V1alpha1PeriodStepStatus {
    #[serde(rename = "firstExecuteTime", skip_serializing_if = "Option::is_none")]
    pub first_execute_time: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lastExecuteTime", skip_serializing_if = "Option::is_none")]
    pub last_execute_time: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl V1alpha1PeriodStepStatus {
    pub fn new(id: String) -> V1alpha1PeriodStepStatus {
        V1alpha1PeriodStepStatus {
            first_execute_time: None,
            id,
            last_execute_time: None,
            message: None,
            name: None,
            phase: None,
            reason: None,
            r#type: None,
        }
    }
}


