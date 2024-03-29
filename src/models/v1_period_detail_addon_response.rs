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
pub struct V1PeriodDetailAddonResponse {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "availableVersions")]
    pub available_versions: Vec<String>,
    #[serde(rename = "definitions")]
    pub definitions: Vec<crate::models::V1PeriodAddonDefinition>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::AddonPeriodDependency>>,
    #[serde(rename = "deployTo", skip_serializing_if = "Option::is_none")]
    pub deploy_to: Option<Box<crate::models::AddonPeriodDeployTo>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "invisible")]
    pub invisible: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "needNamespace", skip_serializing_if = "Option::is_none")]
    pub need_namespace: Option<Vec<String>>,
    #[serde(rename = "registryName", skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "schema")]
    pub schema: String,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Box<crate::models::AddonPeriodSystemRequirements>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "uiSchema")]
    pub ui_schema: Vec<crate::models::SchemaPeriodUiParameter>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "uxPlugins", skip_serializing_if = "Option::is_none")]
    pub ux_plugins: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "version")]
    pub version: String,
}

impl V1PeriodDetailAddonResponse {
    pub fn new(available_versions: Vec<String>, definitions: Vec<crate::models::V1PeriodAddonDefinition>, description: String, icon: String, invisible: bool, name: String, schema: String, ui_schema: Vec<crate::models::SchemaPeriodUiParameter>, version: String) -> V1PeriodDetailAddonResponse {
        V1PeriodDetailAddonResponse {
            annotations: None,
            available_versions,
            definitions,
            dependencies: None,
            deploy_to: None,
            description,
            detail: None,
            icon,
            invisible,
            name,
            need_namespace: None,
            registry_name: None,
            schema,
            system: None,
            tags: None,
            ui_schema,
            url: None,
            ux_plugins: None,
            version,
        }
    }
}


