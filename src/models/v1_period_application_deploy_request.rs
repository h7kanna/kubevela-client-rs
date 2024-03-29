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
pub struct V1PeriodApplicationDeployRequest {
    #[serde(rename = "codeInfo", skip_serializing_if = "Option::is_none")]
    pub code_info: Option<Box<crate::models::ModelPeriodCodeInfo>>,
    #[serde(rename = "force")]
    pub force: bool,
    #[serde(rename = "imageInfo", skip_serializing_if = "Option::is_none")]
    pub image_info: Option<Box<crate::models::ModelPeriodImageInfo>>,
    #[serde(rename = "note")]
    pub note: String,
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
    #[serde(rename = "workflowName")]
    pub workflow_name: String,
}

impl V1PeriodApplicationDeployRequest {
    pub fn new(force: bool, note: String, trigger_type: String, workflow_name: String) -> V1PeriodApplicationDeployRequest {
        V1PeriodApplicationDeployRequest {
            code_info: None,
            force,
            image_info: None,
            note,
            trigger_type,
            workflow_name,
        }
    }
}


