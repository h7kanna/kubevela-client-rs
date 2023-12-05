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
pub struct CommonPeriodHttpOption {
    #[serde(rename = "caFile", skip_serializing_if = "Option::is_none")]
    pub ca_file: Option<String>,
    #[serde(rename = "certFile", skip_serializing_if = "Option::is_none")]
    pub cert_file: Option<String>,
    #[serde(rename = "insecureSkipTLS", skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls: Option<bool>,
    #[serde(rename = "keyFile", skip_serializing_if = "Option::is_none")]
    pub key_file: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl CommonPeriodHttpOption {
    pub fn new() -> CommonPeriodHttpOption {
        CommonPeriodHttpOption {
            ca_file: None,
            cert_file: None,
            insecure_skip_tls: None,
            key_file: None,
            password: None,
            username: None,
        }
    }
}


