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
pub struct CommonPeriodWorkloadGvk {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "kind")]
    pub kind: String,
}

impl CommonPeriodWorkloadGvk {
    pub fn new(api_version: String, kind: String) -> CommonPeriodWorkloadGvk {
        CommonPeriodWorkloadGvk {
            api_version,
            kind,
        }
    }
}


