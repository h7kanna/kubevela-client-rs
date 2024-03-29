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
pub struct V1PeriodUpdateUserRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
}

impl V1PeriodUpdateUserRequest {
    pub fn new(roles: Vec<String>) -> V1PeriodUpdateUserRequest {
        V1PeriodUpdateUserRequest {
            alias: None,
            email: None,
            password: None,
            roles,
        }
    }
}


