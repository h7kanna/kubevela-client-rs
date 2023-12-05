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
pub struct V1PeriodInputVar {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "fromStep")]
    pub from_step: String,
    #[serde(rename = "parameterKey")]
    pub parameter_key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl V1PeriodInputVar {
    pub fn new(from: String, from_step: String, parameter_key: String, value: String) -> V1PeriodInputVar {
        V1PeriodInputVar {
            from,
            from_step,
            parameter_key,
            value,
        }
    }
}


