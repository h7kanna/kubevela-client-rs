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
pub struct V1PeriodRefreshTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

impl V1PeriodRefreshTokenResponse {
    pub fn new(access_token: String, refresh_token: String) -> V1PeriodRefreshTokenResponse {
        V1PeriodRefreshTokenResponse {
            access_token,
            refresh_token,
        }
    }
}

