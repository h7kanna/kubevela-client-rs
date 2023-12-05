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
pub struct V1PeriodListTargetResponse {
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::V1PeriodTargetBase>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl V1PeriodListTargetResponse {
    pub fn new(targets: Vec<crate::models::V1PeriodTargetBase>, total: i64) -> V1PeriodListTargetResponse {
        V1PeriodListTargetResponse {
            targets,
            total,
        }
    }
}


