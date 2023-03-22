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
pub struct ConditionPeriodConditionedStatus {
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::ConditionPeriodCondition>>,
}

impl ConditionPeriodConditionedStatus {
    pub fn new() -> ConditionPeriodConditionedStatus {
        ConditionPeriodConditionedStatus {
            conditions: None,
        }
    }
}


