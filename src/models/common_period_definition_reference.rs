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
pub struct CommonPeriodDefinitionReference {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl CommonPeriodDefinitionReference {
    pub fn new(name: String) -> CommonPeriodDefinitionReference {
        CommonPeriodDefinitionReference {
            name,
            version: None,
        }
    }
}

