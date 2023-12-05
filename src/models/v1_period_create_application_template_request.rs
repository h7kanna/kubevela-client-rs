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
pub struct V1PeriodCreateApplicationTemplateRequest {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "templateName")]
    pub template_name: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl V1PeriodCreateApplicationTemplateRequest {
    pub fn new(description: String, template_name: String, version: String) -> V1PeriodCreateApplicationTemplateRequest {
        V1PeriodCreateApplicationTemplateRequest {
            description,
            template_name,
            version,
        }
    }
}


