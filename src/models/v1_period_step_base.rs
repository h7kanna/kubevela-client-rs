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
pub struct V1PeriodStepBase {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "phase")]
    pub phase: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl V1PeriodStepBase {
    pub fn new(id: String, name: String, phase: String, r#type: String) -> V1PeriodStepBase {
        V1PeriodStepBase {
            id,
            name,
            phase,
            r#type,
        }
    }
}


