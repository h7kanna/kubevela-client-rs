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
pub struct V1PeriodListContextValueResponse {
    #[serde(rename = "contexts")]
    pub contexts: ::std::collections::HashMap<String, Vec<crate::models::ModelPeriodValue>>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl V1PeriodListContextValueResponse {
    pub fn new(contexts: ::std::collections::HashMap<String, Vec<crate::models::ModelPeriodValue>>, total: i32) -> V1PeriodListContextValueResponse {
        V1PeriodListContextValueResponse {
            contexts,
            total,
        }
    }
}


