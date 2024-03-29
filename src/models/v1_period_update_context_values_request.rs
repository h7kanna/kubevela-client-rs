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
pub struct V1PeriodUpdateContextValuesRequest {
    #[serde(rename = "values")]
    pub values: Vec<crate::models::ModelPeriodValue>,
}

impl V1PeriodUpdateContextValuesRequest {
    pub fn new(values: Vec<crate::models::ModelPeriodValue>) -> V1PeriodUpdateContextValuesRequest {
        V1PeriodUpdateContextValuesRequest {
            values,
        }
    }
}


