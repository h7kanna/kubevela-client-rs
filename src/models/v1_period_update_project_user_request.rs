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
pub struct V1PeriodUpdateProjectUserRequest {
    #[serde(rename = "userRoles")]
    pub user_roles: Vec<String>,
}

impl V1PeriodUpdateProjectUserRequest {
    pub fn new(user_roles: Vec<String>) -> V1PeriodUpdateProjectUserRequest {
        V1PeriodUpdateProjectUserRequest {
            user_roles,
        }
    }
}


