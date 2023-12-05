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
pub struct V1PeriodCompareRevisionWithRunningOption {
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

impl V1PeriodCompareRevisionWithRunningOption {
    pub fn new() -> V1PeriodCompareRevisionWithRunningOption {
        V1PeriodCompareRevisionWithRunningOption {
            revision: None,
        }
    }
}


