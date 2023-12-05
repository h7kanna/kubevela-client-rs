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
pub struct ChartPeriodMetadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "appVersion", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::ChartPeriodDependency>>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "home", skip_serializing_if = "Option::is_none")]
    pub home: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "keywords", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "kubeVersion", skip_serializing_if = "Option::is_none")]
    pub kube_version: Option<String>,
    #[serde(rename = "maintainers", skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<Vec<crate::models::ChartPeriodMaintainer>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ChartPeriodMetadata {
    pub fn new() -> ChartPeriodMetadata {
        ChartPeriodMetadata {
            annotations: None,
            api_version: None,
            app_version: None,
            condition: None,
            dependencies: None,
            deprecated: None,
            description: None,
            home: None,
            icon: None,
            keywords: None,
            kube_version: None,
            maintainers: None,
            name: None,
            sources: None,
            tags: None,
            r#type: None,
            version: None,
        }
    }
}


