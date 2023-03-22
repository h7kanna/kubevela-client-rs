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
pub struct V1PeriodAccessKeyRequest {
    #[serde(rename = "accessKeyID")]
    pub access_key_id: String,
    #[serde(rename = "accessKeySecret")]
    pub access_key_secret: String,
}

impl V1PeriodAccessKeyRequest {
    pub fn new(access_key_id: String, access_key_secret: String) -> V1PeriodAccessKeyRequest {
        V1PeriodAccessKeyRequest {
            access_key_id,
            access_key_secret,
        }
    }
}

