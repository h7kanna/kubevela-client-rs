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
pub struct V1PeriodCreateCloudClusterResponse {
    #[serde(rename = "clusterID")]
    pub cluster_id: String,
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl V1PeriodCreateCloudClusterResponse {
    pub fn new(cluster_id: String, cluster_name: String, status: String) -> V1PeriodCreateCloudClusterResponse {
        V1PeriodCreateCloudClusterResponse {
            cluster_id,
            cluster_name,
            status,
        }
    }
}


