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
pub struct V1PeriodListAddonResponse {
    #[serde(rename = "addons")]
    pub addons: Vec<crate::models::V1PeriodAddonInfo>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl V1PeriodListAddonResponse {
    pub fn new(addons: Vec<crate::models::V1PeriodAddonInfo>) -> V1PeriodListAddonResponse {
        V1PeriodListAddonResponse {
            addons,
            message: None,
        }
    }
}


