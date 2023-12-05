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
pub struct V1PeriodAddonInfo {
    #[serde(rename = "Meta")]
    pub meta: Box<crate::models::AddonPeriodMeta>,
    #[serde(rename = "registryName")]
    pub registry_name: String,
}

impl V1PeriodAddonInfo {
    pub fn new(meta: crate::models::AddonPeriodMeta, registry_name: String) -> V1PeriodAddonInfo {
        V1PeriodAddonInfo {
            meta: Box::new(meta),
            registry_name,
        }
    }
}


