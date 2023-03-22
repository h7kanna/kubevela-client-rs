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
pub struct V1PeriodDefinitionBase {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<Box<crate::models::V1beta1PeriodComponentDefinitionSpec>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ownerAddon")]
    pub owner_addon: String,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<crate::models::V1beta1PeriodPolicyDefinitionSpec>>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "trait", skip_serializing_if = "Option::is_none")]
    pub r#trait: Option<Box<crate::models::V1beta1PeriodTraitDefinitionSpec>>,
    #[serde(rename = "workflowStep", skip_serializing_if = "Option::is_none")]
    pub workflow_step: Option<Box<crate::models::V1beta1PeriodWorkflowStepDefinitionSpec>>,
    #[serde(rename = "workloadType", skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
}

impl V1PeriodDefinitionBase {
    pub fn new(alias: String, description: String, icon: String, labels: ::std::collections::HashMap<String, String>, name: String, owner_addon: String, status: String) -> V1PeriodDefinitionBase {
        V1PeriodDefinitionBase {
            alias,
            component: None,
            description,
            icon,
            labels,
            name,
            owner_addon,
            policy: None,
            status,
            r#trait: None,
            workflow_step: None,
            workload_type: None,
        }
    }
}


