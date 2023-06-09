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
pub struct V1PeriodListProjectResponse {
    #[serde(rename = "projects")]
    pub projects: Vec<crate::models::V1PeriodProjectBase>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl V1PeriodListProjectResponse {
    pub fn new(projects: Vec<crate::models::V1PeriodProjectBase>, total: i64) -> V1PeriodListProjectResponse {
        V1PeriodListProjectResponse {
            projects,
            total,
        }
    }
}


