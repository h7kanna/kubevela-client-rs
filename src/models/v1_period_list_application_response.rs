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
pub struct V1PeriodListApplicationResponse {
    #[serde(rename = "applications")]
    pub applications: Vec<crate::models::V1PeriodApplicationBase>,
}

impl V1PeriodListApplicationResponse {
    pub fn new(applications: Vec<crate::models::V1PeriodApplicationBase>) -> V1PeriodListApplicationResponse {
        V1PeriodListApplicationResponse {
            applications,
        }
    }
}


