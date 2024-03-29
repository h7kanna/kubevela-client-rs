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
pub struct V1alpha1PeriodInputItem {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "parameterKey", skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
}

impl V1alpha1PeriodInputItem {
    pub fn new(from: String) -> V1alpha1PeriodInputItem {
        V1alpha1PeriodInputItem {
            from,
            parameter_key: None,
        }
    }
}


