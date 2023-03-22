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
pub struct V1PeriodListConfigDistributionResponse {
    #[serde(rename = "distributions")]
    pub distributions: Vec<crate::models::ConfigPeriodDistribution>,
}

impl V1PeriodListConfigDistributionResponse {
    pub fn new(distributions: Vec<crate::models::ConfigPeriodDistribution>) -> V1PeriodListConfigDistributionResponse {
        V1PeriodListConfigDistributionResponse {
            distributions,
        }
    }
}

