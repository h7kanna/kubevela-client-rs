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
pub struct V1PeriodDetailWorkflowRecordResponse {
    #[serde(rename = "applicationRevision")]
    pub application_revision: String,
    #[serde(rename = "deployTime")]
    pub deploy_time: String,
    #[serde(rename = "deployUser")]
    pub deploy_user: String,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "note")]
    pub note: String,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<crate::models::ModelPeriodWorkflowStepStatus>>,
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
    #[serde(rename = "workflowAlias")]
    pub workflow_alias: String,
    #[serde(rename = "workflowName")]
    pub workflow_name: String,
}

impl V1PeriodDetailWorkflowRecordResponse {
    pub fn new(application_revision: String, deploy_time: String, deploy_user: String, message: String, mode: String, name: String, namespace: String, note: String, status: String, trigger_type: String, workflow_alias: String, workflow_name: String) -> V1PeriodDetailWorkflowRecordResponse {
        V1PeriodDetailWorkflowRecordResponse {
            application_revision,
            deploy_time,
            deploy_user,
            end_time: None,
            message,
            mode,
            name,
            namespace,
            note,
            start_time: None,
            status,
            steps: None,
            trigger_type,
            workflow_alias,
            workflow_name,
        }
    }
}


