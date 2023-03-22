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
pub struct CommonPeriodWorkflowStatus {
    #[serde(rename = "appRevision", skip_serializing_if = "Option::is_none")]
    pub app_revision: Option<String>,
    #[serde(rename = "contextBackend", skip_serializing_if = "Option::is_none")]
    pub context_backend: Option<Box<crate::models::V1PeriodObjectReference>>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "finished")]
    pub finished: bool,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<crate::models::V1alpha1PeriodWorkflowStepStatus>>,
    #[serde(rename = "suspend")]
    pub suspend: bool,
    #[serde(rename = "suspendState", skip_serializing_if = "Option::is_none")]
    pub suspend_state: Option<String>,
    #[serde(rename = "terminated")]
    pub terminated: bool,
}

impl CommonPeriodWorkflowStatus {
    pub fn new(finished: bool, mode: String, suspend: bool, terminated: bool) -> CommonPeriodWorkflowStatus {
        CommonPeriodWorkflowStatus {
            app_revision: None,
            context_backend: None,
            end_time: None,
            finished,
            message: None,
            mode,
            start_time: None,
            status: None,
            steps: None,
            suspend,
            suspend_state: None,
            terminated,
        }
    }
}

