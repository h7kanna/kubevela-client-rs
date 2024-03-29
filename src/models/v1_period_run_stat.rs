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
pub struct V1PeriodRunStat {
    #[serde(rename = "activeNum")]
    pub active_num: i32,
    #[serde(rename = "total")]
    pub total: Box<crate::models::V1PeriodRunStatInfo>,
    #[serde(rename = "week")]
    pub week: Vec<crate::models::V1PeriodRunStatInfo>,
}

impl V1PeriodRunStat {
    pub fn new(active_num: i32, total: crate::models::V1PeriodRunStatInfo, week: Vec<crate::models::V1PeriodRunStatInfo>) -> V1PeriodRunStat {
        V1PeriodRunStat {
            active_num,
            total: Box::new(total),
            week,
        }
    }
}


