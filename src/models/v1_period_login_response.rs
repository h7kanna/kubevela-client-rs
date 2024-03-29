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
pub struct V1PeriodLoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "user")]
    pub user: Box<crate::models::V1PeriodUserBase>,
}

impl V1PeriodLoginResponse {
    pub fn new(access_token: String, refresh_token: String, user: crate::models::V1PeriodUserBase) -> V1PeriodLoginResponse {
        V1PeriodLoginResponse {
            access_token,
            refresh_token,
            user: Box::new(user),
        }
    }
}


