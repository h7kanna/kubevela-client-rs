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
pub struct V1PeriodListEnvResponse {
    #[serde(rename = "envs")]
    pub envs: Vec<crate::models::V1PeriodEnv>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl V1PeriodListEnvResponse {
    pub fn new(envs: Vec<crate::models::V1PeriodEnv>, total: i64) -> V1PeriodListEnvResponse {
        V1PeriodListEnvResponse {
            envs,
            total,
        }
    }
}


