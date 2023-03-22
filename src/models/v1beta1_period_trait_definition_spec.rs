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
pub struct V1beta1PeriodTraitDefinitionSpec {
    #[serde(rename = "appliesToWorkloads", skip_serializing_if = "Option::is_none")]
    pub applies_to_workloads: Option<Vec<String>>,
    #[serde(rename = "conflictsWith", skip_serializing_if = "Option::is_none")]
    pub conflicts_with: Option<Vec<String>>,
    #[serde(rename = "controlPlaneOnly", skip_serializing_if = "Option::is_none")]
    pub control_plane_only: Option<bool>,
    #[serde(rename = "definitionRef", skip_serializing_if = "Option::is_none")]
    pub definition_ref: Option<Box<crate::models::CommonPeriodDefinitionReference>>,
    #[serde(rename = "extension", skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "manageWorkload", skip_serializing_if = "Option::is_none")]
    pub manage_workload: Option<bool>,
    #[serde(rename = "podDisruptive", skip_serializing_if = "Option::is_none")]
    pub pod_disruptive: Option<bool>,
    #[serde(rename = "revisionEnabled", skip_serializing_if = "Option::is_none")]
    pub revision_enabled: Option<bool>,
    #[serde(rename = "schematic", skip_serializing_if = "Option::is_none")]
    pub schematic: Option<Box<crate::models::CommonPeriodSchematic>>,
    #[serde(rename = "stage", skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::CommonPeriodStatus>>,
    #[serde(rename = "workloadRefPath", skip_serializing_if = "Option::is_none")]
    pub workload_ref_path: Option<String>,
}

impl V1beta1PeriodTraitDefinitionSpec {
    pub fn new() -> V1beta1PeriodTraitDefinitionSpec {
        V1beta1PeriodTraitDefinitionSpec {
            applies_to_workloads: None,
            conflicts_with: None,
            control_plane_only: None,
            definition_ref: None,
            extension: None,
            manage_workload: None,
            pod_disruptive: None,
            revision_enabled: None,
            schematic: None,
            stage: None,
            status: None,
            workload_ref_path: None,
        }
    }
}


