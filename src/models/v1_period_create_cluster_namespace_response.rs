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
pub struct V1PeriodCreateClusterNamespaceResponse {
    #[serde(rename = "exists")]
    pub exists: bool,
}

impl V1PeriodCreateClusterNamespaceResponse {
    pub fn new(exists: bool) -> V1PeriodCreateClusterNamespaceResponse {
        V1PeriodCreateClusterNamespaceResponse {
            exists,
        }
    }
}


