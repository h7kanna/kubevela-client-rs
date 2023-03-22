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
pub struct V1PeriodHandleApplicationTriggerWebhookRequest {
    #[serde(rename = "codeInfo", skip_serializing_if = "Option::is_none")]
    pub code_info: Option<Box<crate::models::ModelPeriodCodeInfo>>,
    #[serde(rename = "upgrade", skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl V1PeriodHandleApplicationTriggerWebhookRequest {
    pub fn new() -> V1PeriodHandleApplicationTriggerWebhookRequest {
        V1PeriodHandleApplicationTriggerWebhookRequest {
            code_info: None,
            upgrade: None,
        }
    }
}


