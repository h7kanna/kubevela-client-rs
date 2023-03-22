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
pub struct V1PeriodNamespacedName {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl V1PeriodNamespacedName {
    pub fn new(name: String) -> V1PeriodNamespacedName {
        V1PeriodNamespacedName {
            name,
            namespace: None,
        }
    }
}


