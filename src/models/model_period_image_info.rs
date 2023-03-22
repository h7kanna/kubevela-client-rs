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
pub struct ModelPeriodImageInfo {
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<crate::models::ModelPeriodImageRepository>>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<crate::models::ModelPeriodImageResource>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ModelPeriodImageInfo {
    pub fn new(r#type: String) -> ModelPeriodImageInfo {
        ModelPeriodImageInfo {
            repository: None,
            resource: None,
            r#type,
        }
    }
}


