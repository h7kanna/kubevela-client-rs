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
pub struct ConfigPeriodNamespacedName {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
}

impl ConfigPeriodNamespacedName {
    pub fn new(name: String, namespace: String) -> ConfigPeriodNamespacedName {
        ConfigPeriodNamespacedName {
            name,
            namespace,
        }
    }
}


