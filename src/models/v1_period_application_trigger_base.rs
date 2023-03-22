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
pub struct V1PeriodApplicationTriggerBase {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "componentName", skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "payloadType")]
    pub payload_type: String,
    #[serde(rename = "registry")]
    pub registry: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "workflowName")]
    pub workflow_name: String,
}

impl V1PeriodApplicationTriggerBase {
    pub fn new(create_time: String, name: String, payload_type: String, registry: String, token: String, r#type: String, update_time: String, workflow_name: String) -> V1PeriodApplicationTriggerBase {
        V1PeriodApplicationTriggerBase {
            alias: None,
            component_name: None,
            create_time,
            description: None,
            name,
            payload_type,
            registry,
            token,
            r#type,
            update_time,
            workflow_name,
        }
    }
}


