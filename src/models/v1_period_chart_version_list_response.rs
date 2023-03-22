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
pub struct V1PeriodChartVersionListResponse {
    #[serde(rename = "versions")]
    pub versions: Vec<crate::models::RepoPeriodChartVersion>,
}

impl V1PeriodChartVersionListResponse {
    pub fn new(versions: Vec<crate::models::RepoPeriodChartVersion>) -> V1PeriodChartVersionListResponse {
        V1PeriodChartVersionListResponse {
            versions,
        }
    }
}

