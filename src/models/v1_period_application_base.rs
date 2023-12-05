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
pub struct V1PeriodApplicationBase {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "project")]
    pub project: Box<crate::models::V1PeriodProjectBase>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "updateTime")]
    pub update_time: String,
}

impl V1PeriodApplicationBase {
    pub fn new(alias: String, create_time: String, description: String, icon: String, name: String, project: crate::models::V1PeriodProjectBase, update_time: String) -> V1PeriodApplicationBase {
        V1PeriodApplicationBase {
            alias,
            annotations: None,
            create_time,
            description,
            icon,
            labels: None,
            name,
            project: Box::new(project),
            read_only: None,
            update_time,
        }
    }
}


