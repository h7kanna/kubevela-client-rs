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
pub struct V1PeriodGetPipelineRunOutputResponse {
    #[serde(rename = "outputs")]
    pub outputs: Vec<crate::models::V1PeriodStepOutputBase>,
}

impl V1PeriodGetPipelineRunOutputResponse {
    pub fn new(outputs: Vec<crate::models::V1PeriodStepOutputBase>) -> V1PeriodGetPipelineRunOutputResponse {
        V1PeriodGetPipelineRunOutputResponse {
            outputs,
        }
    }
}


