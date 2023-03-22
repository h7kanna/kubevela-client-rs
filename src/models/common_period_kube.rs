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
pub struct CommonPeriodKube {
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::CommonPeriodKubeParameter>>,
    #[serde(rename = "template")]
    pub template: String,
}

impl CommonPeriodKube {
    pub fn new(template: String) -> CommonPeriodKube {
        CommonPeriodKube {
            parameters: None,
            template,
        }
    }
}


