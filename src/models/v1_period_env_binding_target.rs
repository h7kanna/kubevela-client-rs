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
pub struct V1PeriodEnvBindingTarget {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Box<crate::models::V1PeriodClusterTarget>>,
    #[serde(rename = "name")]
    pub name: String,
}

impl V1PeriodEnvBindingTarget {
    pub fn new(alias: String, name: String) -> V1PeriodEnvBindingTarget {
        V1PeriodEnvBindingTarget {
            alias,
            cluster: None,
            name,
        }
    }
}


