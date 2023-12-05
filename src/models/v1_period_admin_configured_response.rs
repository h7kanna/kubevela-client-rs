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
pub struct V1PeriodAdminConfiguredResponse {
    #[serde(rename = "configured")]
    pub configured: bool,
}

impl V1PeriodAdminConfiguredResponse {
    pub fn new(configured: bool) -> V1PeriodAdminConfiguredResponse {
        V1PeriodAdminConfiguredResponse {
            configured,
        }
    }
}

