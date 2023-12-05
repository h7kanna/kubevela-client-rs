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
pub struct SchemaPeriodValidate {
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(rename = "immutable")]
    pub immutable: bool,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::SchemaPeriodOption>>,
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl SchemaPeriodValidate {
    pub fn new(immutable: bool) -> SchemaPeriodValidate {
        SchemaPeriodValidate {
            default_value: None,
            immutable,
            max: None,
            max_length: None,
            min: None,
            min_length: None,
            options: None,
            pattern: None,
            required: None,
        }
    }
}


