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
pub struct V1PeriodHealthConfig {
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "Retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(rename = "StartPeriod", skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
    #[serde(rename = "Test", skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl V1PeriodHealthConfig {
    pub fn new() -> V1PeriodHealthConfig {
        V1PeriodHealthConfig {
            interval: None,
            retries: None,
            start_period: None,
            test: None,
            timeout: None,
        }
    }
}

