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
pub struct V1PeriodPermissionBase {
    #[serde(rename = "actions")]
    pub actions: Vec<String>,
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "resources")]
    pub resources: Vec<String>,
    #[serde(rename = "updateTime")]
    pub update_time: String,
}

impl V1PeriodPermissionBase {
    pub fn new(actions: Vec<String>, alias: String, create_time: String, effect: String, name: String, resources: Vec<String>, update_time: String) -> V1PeriodPermissionBase {
        V1PeriodPermissionBase {
            actions,
            alias,
            create_time,
            effect,
            name,
            resources,
            update_time,
        }
    }
}


