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
pub struct AddonPeriodMeta {
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::AddonPeriodDependency>>,
    #[serde(rename = "deployTo", skip_serializing_if = "Option::is_none")]
    pub deploy_to: Option<Box<crate::models::AddonPeriodDeployTo>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "invisible")]
    pub invisible: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "needNamespace", skip_serializing_if = "Option::is_none")]
    pub need_namespace: Option<Vec<String>>,
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Box<crate::models::AddonPeriodSystemRequirements>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "version")]
    pub version: String,
}

impl AddonPeriodMeta {
    pub fn new(description: String, icon: String, invisible: bool, name: String, version: String) -> AddonPeriodMeta {
        AddonPeriodMeta {
            dependencies: None,
            deploy_to: None,
            description,
            icon,
            invisible,
            name,
            need_namespace: None,
            system: None,
            tags: None,
            url: None,
            version,
        }
    }
}

