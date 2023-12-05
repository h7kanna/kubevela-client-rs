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
pub struct V1PeriodListRolesResponse {
    #[serde(rename = "roles")]
    pub roles: Vec<crate::models::V1PeriodRoleBase>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl V1PeriodListRolesResponse {
    pub fn new(roles: Vec<crate::models::V1PeriodRoleBase>, total: i64) -> V1PeriodListRolesResponse {
        V1PeriodListRolesResponse {
            roles,
            total,
        }
    }
}


