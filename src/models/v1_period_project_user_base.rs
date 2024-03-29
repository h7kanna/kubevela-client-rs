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
pub struct V1PeriodProjectUserBase {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "userRoles")]
    pub user_roles: Vec<String>,
}

impl V1PeriodProjectUserBase {
    pub fn new(alias: String, create_time: String, name: String, update_time: String, user_roles: Vec<String>) -> V1PeriodProjectUserBase {
        V1PeriodProjectUserBase {
            alias,
            create_time,
            name,
            update_time,
            user_roles,
        }
    }
}


