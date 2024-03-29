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
pub struct V1PeriodImageRegistry {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

impl V1PeriodImageRegistry {
    pub fn new(domain: String, name: String, secret_name: String) -> V1PeriodImageRegistry {
        V1PeriodImageRegistry {
            domain,
            name,
            secret_name,
        }
    }
}


