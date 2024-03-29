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
pub struct BcodePeriodBcode {
    #[serde(rename = "BusinessCode")]
    pub business_code: i32,
    #[serde(rename = "Message")]
    pub message: String,
}

impl BcodePeriodBcode {
    pub fn new(business_code: i32, message: String) -> BcodePeriodBcode {
        BcodePeriodBcode {
            business_code,
            message,
        }
    }
}


