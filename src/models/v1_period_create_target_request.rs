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
pub struct V1PeriodCreateTargetRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Box<crate::models::V1PeriodClusterTarget>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "variable", skip_serializing_if = "Option::is_none")]
    pub variable: Option<serde_json::Value>,
}

impl V1PeriodCreateTargetRequest {
    pub fn new(name: String, project: String) -> V1PeriodCreateTargetRequest {
        V1PeriodCreateTargetRequest {
            alias: None,
            cluster: None,
            description: None,
            name,
            project,
            variable: None,
        }
    }
}

