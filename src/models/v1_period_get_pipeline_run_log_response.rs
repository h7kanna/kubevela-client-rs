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
pub struct V1PeriodGetPipelineRunLogResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "log")]
    pub log: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "phase")]
    pub phase: String,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl V1PeriodGetPipelineRunLogResponse {
    pub fn new(id: String, log: String, name: String, phase: String, source: String, r#type: String) -> V1PeriodGetPipelineRunLogResponse {
        V1PeriodGetPipelineRunLogResponse {
            id,
            log,
            name,
            phase,
            source,
            r#type,
        }
    }
}

