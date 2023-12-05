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
pub struct CommonPeriodPolicyStatus {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CommonPeriodPolicyStatus {
    pub fn new(name: String, r#type: String) -> CommonPeriodPolicyStatus {
        CommonPeriodPolicyStatus {
            name,
            status: None,
            r#type,
        }
    }
}


