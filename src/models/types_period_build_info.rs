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
pub struct TypesPeriodBuildInfo {
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "repo", skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

impl TypesPeriodBuildInfo {
    pub fn new() -> TypesPeriodBuildInfo {
        TypesPeriodBuildInfo {
            branch: None,
            hash: None,
            repo: None,
            time: None,
        }
    }
}


