/*
 * Kubevela api doc
 *
 * Kubevela api doc
 *
 * The version of the OpenAPI document: 1.9.7
 * Contact: feedback@mail.kubevela.io
 * Generated by: https://openapi-generator.tech
 */

/// V1PeriodPolicyRule : PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1PeriodPolicyRule {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed. \"\" represents the core API group and \"*\" represents all API groups.
    #[serde(rename = "apiGroups", skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<Vec<String>>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as \"pods\" or \"secrets\") or non-resource URL paths (such as \"/api\"),  but not both.
    #[serde(rename = "nonResourceURLs", skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<Vec<String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[serde(rename = "resourceNames", skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<Vec<String>>,
    /// Resources is a list of resources this rule applies to. '*' represents all resources.
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
    #[serde(rename = "verbs")]
    pub verbs: Vec<String>,
}

impl V1PeriodPolicyRule {
    /// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
    pub fn new(verbs: Vec<String>) -> V1PeriodPolicyRule {
        V1PeriodPolicyRule {
            api_groups: None,
            non_resource_urls: None,
            resource_names: None,
            resources: None,
            verbs,
        }
    }
}


