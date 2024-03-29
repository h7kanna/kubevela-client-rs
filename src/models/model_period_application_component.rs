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
pub struct ModelPeriodApplicationComponent {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "appPrimaryKey")]
    pub app_primary_key: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "dependsOn", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalRevision", skip_serializing_if = "Option::is_none")]
    pub external_revision: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<crate::models::V1alpha1PeriodInputItem>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "main")]
    pub main: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<crate::models::V1alpha1PeriodOutputItem>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<crate::models::ModelPeriodApplicationTrait>>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "workloadType", skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<Box<crate::models::CommonPeriodWorkloadTypeDescriptor>>,
}

impl ModelPeriodApplicationComponent {
    pub fn new(alias: String, app_primary_key: String, create_time: String, creator: String, main: bool, name: String, r#type: String, update_time: String) -> ModelPeriodApplicationComponent {
        ModelPeriodApplicationComponent {
            alias,
            app_primary_key,
            create_time,
            creator,
            depends_on: None,
            description: None,
            external_revision: None,
            icon: None,
            inputs: None,
            labels: None,
            main,
            name,
            outputs: None,
            properties: None,
            scopes: None,
            traits: None,
            r#type,
            update_time,
            workload_type: None,
        }
    }
}


