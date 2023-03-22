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
pub struct V1PeriodComponentBase {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "componentType")]
    pub component_type: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,
    #[serde(rename = "description")]
    pub description: String,
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
    #[serde(rename = "traits")]
    pub traits: Vec<crate::models::V1PeriodApplicationTrait>,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "workloadType", skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<Box<crate::models::CommonPeriodWorkloadTypeDescriptor>>,
}

impl V1PeriodComponentBase {
    pub fn new(alias: String, component_type: String, create_time: String, depends_on: Vec<String>, description: String, main: bool, name: String, traits: Vec<crate::models::V1PeriodApplicationTrait>, update_time: String) -> V1PeriodComponentBase {
        V1PeriodComponentBase {
            alias,
            component_type,
            create_time,
            creator: None,
            depends_on,
            description,
            icon: None,
            inputs: None,
            labels: None,
            main,
            name,
            outputs: None,
            traits,
            update_time,
            workload_type: None,
        }
    }
}

