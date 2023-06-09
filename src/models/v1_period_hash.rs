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
pub struct V1PeriodHash {
    #[serde(rename = "Algorithm")]
    pub algorithm: String,
    #[serde(rename = "Hex")]
    pub hex: String,
}

impl V1PeriodHash {
    pub fn new(algorithm: String, hex: String) -> V1PeriodHash {
        V1PeriodHash {
            algorithm,
            hex,
        }
    }
}


