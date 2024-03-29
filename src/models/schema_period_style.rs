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
pub struct SchemaPeriodStyle {
    #[serde(rename = "colSpan")]
    pub col_span: i32,
}

impl SchemaPeriodStyle {
    pub fn new(col_span: i32) -> SchemaPeriodStyle {
        SchemaPeriodStyle {
            col_span,
        }
    }
}


