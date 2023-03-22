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
pub struct V1PeriodApplicationRevisionBase {
    #[serde(rename = "codeInfo", skip_serializing_if = "Option::is_none")]
    pub code_info: Option<Box<crate::models::ModelPeriodCodeInfo>>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "deployUser", skip_serializing_if = "Option::is_none")]
    pub deploy_user: Option<Box<crate::models::V1PeriodNameAlias>>,
    #[serde(rename = "envName")]
    pub env_name: String,
    #[serde(rename = "imageInfo", skip_serializing_if = "Option::is_none")]
    pub image_info: Option<Box<crate::models::ModelPeriodImageInfo>>,
    #[serde(rename = "note")]
    pub note: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "workflowName")]
    pub workflow_name: String,
}

impl V1PeriodApplicationRevisionBase {
    pub fn new(create_time: String, env_name: String, note: String, status: String, trigger_type: String, version: String, workflow_name: String) -> V1PeriodApplicationRevisionBase {
        V1PeriodApplicationRevisionBase {
            code_info: None,
            create_time,
            deploy_user: None,
            env_name,
            image_info: None,
            note,
            reason: None,
            status,
            trigger_type,
            version,
            workflow_name,
        }
    }
}

