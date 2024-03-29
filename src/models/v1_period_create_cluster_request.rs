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
pub struct V1PeriodCreateClusterRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "dashboardURL", skip_serializing_if = "Option::is_none")]
    pub dashboard_url: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "kubeConfig", skip_serializing_if = "Option::is_none")]
    pub kube_config: Option<String>,
    #[serde(rename = "kubeConfigSecret", skip_serializing_if = "Option::is_none")]
    pub kube_config_secret: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name")]
    pub name: String,
}

impl V1PeriodCreateClusterRequest {
    pub fn new(icon: String, name: String) -> V1PeriodCreateClusterRequest {
        V1PeriodCreateClusterRequest {
            alias: None,
            dashboard_url: None,
            description: None,
            icon,
            kube_config: None,
            kube_config_secret: None,
            labels: None,
            name,
        }
    }
}


