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
pub struct V1PeriodCreateUserRequest {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
}

impl V1PeriodCreateUserRequest {
    pub fn new(email: String, name: String, password: String, roles: Vec<String>) -> V1PeriodCreateUserRequest {
        V1PeriodCreateUserRequest {
            alias: None,
            email,
            name,
            password,
            roles,
        }
    }
}


