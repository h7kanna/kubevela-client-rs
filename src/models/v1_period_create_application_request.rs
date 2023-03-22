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
pub struct V1PeriodCreateApplicationRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "component")]
    pub component: Box<crate::models::V1PeriodCreateComponentRequest>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "envBinding", skip_serializing_if = "Option::is_none")]
    pub env_binding: Option<Vec<crate::models::V1PeriodEnvBinding>>,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "project")]
    pub project: String,
}

impl V1PeriodCreateApplicationRequest {
    pub fn new(component: crate::models::V1PeriodCreateComponentRequest, icon: String, name: String, project: String) -> V1PeriodCreateApplicationRequest {
        V1PeriodCreateApplicationRequest {
            alias: None,
            component: Box::new(component),
            description: None,
            env_binding: None,
            icon,
            labels: None,
            name,
            project,
        }
    }
}


