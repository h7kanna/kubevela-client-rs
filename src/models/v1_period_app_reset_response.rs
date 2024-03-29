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
pub struct V1PeriodAppResetResponse {
    #[serde(rename = "isReset")]
    pub is_reset: bool,
}

impl V1PeriodAppResetResponse {
    pub fn new(is_reset: bool) -> V1PeriodAppResetResponse {
        V1PeriodAppResetResponse {
            is_reset,
        }
    }
}


