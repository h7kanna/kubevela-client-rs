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
pub struct TypesPeriodHeader {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl TypesPeriodHeader {
    pub fn new(name: String, value: String) -> TypesPeriodHeader {
        TypesPeriodHeader {
            name,
            value,
        }
    }
}

