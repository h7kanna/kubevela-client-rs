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
pub struct V1PeriodTargetBase {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "appNum", skip_serializing_if = "Option::is_none")]
    pub app_num: Option<i64>,
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Box<crate::models::V1PeriodClusterTarget>>,
    #[serde(rename = "clusterAlias", skip_serializing_if = "Option::is_none")]
    pub cluster_alias: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "project")]
    pub project: Box<crate::models::V1PeriodNameAlias>,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "variable", skip_serializing_if = "Option::is_none")]
    pub variable: Option<serde_json::Value>,
}

impl V1PeriodTargetBase {
    pub fn new(create_time: String, name: String, project: crate::models::V1PeriodNameAlias, update_time: String) -> V1PeriodTargetBase {
        V1PeriodTargetBase {
            alias: None,
            app_num: None,
            cluster: None,
            cluster_alias: None,
            create_time,
            description: None,
            name,
            project: Box::new(project),
            update_time,
            variable: None,
        }
    }
}


