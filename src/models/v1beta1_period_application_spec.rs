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
pub struct V1beta1PeriodApplicationSpec {
    #[serde(rename = "components")]
    pub components: Vec<crate::models::CommonPeriodApplicationComponent>,
    #[serde(rename = "policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::models::V1beta1PeriodAppPolicy>>,
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Box<crate::models::V1beta1PeriodWorkflow>>,
}

impl V1beta1PeriodApplicationSpec {
    pub fn new(components: Vec<crate::models::CommonPeriodApplicationComponent>) -> V1beta1PeriodApplicationSpec {
        V1beta1PeriodApplicationSpec {
            components,
            policies: None,
            workflow: None,
        }
    }
}


