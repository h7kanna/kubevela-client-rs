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
pub struct V1PeriodListCloudClusterCreationResponse {
    #[serde(rename = "creations")]
    pub creations: Vec<crate::models::V1PeriodCreateCloudClusterResponse>,
}

impl V1PeriodListCloudClusterCreationResponse {
    pub fn new(creations: Vec<crate::models::V1PeriodCreateCloudClusterResponse>) -> V1PeriodListCloudClusterCreationResponse {
        V1PeriodListCloudClusterCreationResponse {
            creations,
        }
    }
}


