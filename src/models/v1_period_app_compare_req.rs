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
pub struct V1PeriodAppCompareReq {
    #[serde(rename = "compareLatestWithRunning", skip_serializing_if = "Option::is_none")]
    pub compare_latest_with_running: Option<Box<crate::models::V1PeriodCompareLatestWithRunningOption>>,
    #[serde(rename = "compareRevisionWithLatest", skip_serializing_if = "Option::is_none")]
    pub compare_revision_with_latest: Option<Box<crate::models::V1PeriodCompareRevisionWithLatestOption>>,
    #[serde(rename = "compareRevisionWithRunning", skip_serializing_if = "Option::is_none")]
    pub compare_revision_with_running: Option<Box<crate::models::V1PeriodCompareRevisionWithRunningOption>>,
}

impl V1PeriodAppCompareReq {
    pub fn new() -> V1PeriodAppCompareReq {
        V1PeriodAppCompareReq {
            compare_latest_with_running: None,
            compare_revision_with_latest: None,
            compare_revision_with_running: None,
        }
    }
}


