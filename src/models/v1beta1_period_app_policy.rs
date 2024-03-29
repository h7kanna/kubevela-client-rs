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
pub struct V1beta1PeriodAppPolicy {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl V1beta1PeriodAppPolicy {
    pub fn new(r#type: String) -> V1beta1PeriodAppPolicy {
        V1beta1PeriodAppPolicy {
            name: None,
            properties: None,
            r#type,
        }
    }
}


