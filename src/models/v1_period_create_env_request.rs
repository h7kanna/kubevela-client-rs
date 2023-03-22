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
pub struct V1PeriodCreateEnvRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "allowTargetConflict", skip_serializing_if = "Option::is_none")]
    pub allow_target_conflict: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

impl V1PeriodCreateEnvRequest {
    pub fn new(name: String, namespace: String, project: String) -> V1PeriodCreateEnvRequest {
        V1PeriodCreateEnvRequest {
            alias: None,
            allow_target_conflict: None,
            description: None,
            name,
            namespace,
            project,
            targets: None,
        }
    }
}

