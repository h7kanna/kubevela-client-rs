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
pub struct V1PeriodApplicationRollbackResponse {
    #[serde(rename = "record")]
    pub record: Box<crate::models::V1PeriodWorkflowRecordBase>,
}

impl V1PeriodApplicationRollbackResponse {
    pub fn new(record: crate::models::V1PeriodWorkflowRecordBase) -> V1PeriodApplicationRollbackResponse {
        V1PeriodApplicationRollbackResponse {
            record: Box::new(record),
        }
    }
}


