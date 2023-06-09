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
pub struct V1PeriodCompareRevisionWithLatestOption {
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

impl V1PeriodCompareRevisionWithLatestOption {
    pub fn new() -> V1PeriodCompareRevisionWithLatestOption {
        V1PeriodCompareRevisionWithLatestOption {
            revision: None,
        }
    }
}


