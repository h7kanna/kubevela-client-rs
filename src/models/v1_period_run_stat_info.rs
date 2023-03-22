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
pub struct V1PeriodRunStatInfo {
    #[serde(rename = "fail")]
    pub fail: i32,
    #[serde(rename = "success")]
    pub success: i32,
    #[serde(rename = "total")]
    pub total: i32,
}

impl V1PeriodRunStatInfo {
    pub fn new(fail: i32, success: i32, total: i32) -> V1PeriodRunStatInfo {
        V1PeriodRunStatInfo {
            fail,
            success,
            total,
        }
    }
}


