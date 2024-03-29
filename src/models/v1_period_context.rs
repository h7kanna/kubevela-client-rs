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
pub struct V1PeriodContext {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "values")]
    pub values: Vec<crate::models::ModelPeriodValue>,
}

impl V1PeriodContext {
    pub fn new(name: String, values: Vec<crate::models::ModelPeriodValue>) -> V1PeriodContext {
        V1PeriodContext {
            name,
            values,
        }
    }
}


