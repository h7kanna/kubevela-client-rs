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
pub struct V1PeriodListWorkflowRecordsResponse {
    #[serde(rename = "records")]
    pub records: Vec<crate::models::V1PeriodWorkflowRecord>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl V1PeriodListWorkflowRecordsResponse {
    pub fn new(records: Vec<crate::models::V1PeriodWorkflowRecord>, total: i64) -> V1PeriodListWorkflowRecordsResponse {
        V1PeriodListWorkflowRecordsResponse {
            records,
            total,
        }
    }
}


