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
pub struct V1PeriodListConfigTemplateResponse {
    #[serde(rename = "templates")]
    pub templates: Vec<crate::models::V1PeriodConfigTemplate>,
}

impl V1PeriodListConfigTemplateResponse {
    pub fn new(templates: Vec<crate::models::V1PeriodConfigTemplate>) -> V1PeriodListConfigTemplateResponse {
        V1PeriodListConfigTemplateResponse {
            templates,
        }
    }
}


