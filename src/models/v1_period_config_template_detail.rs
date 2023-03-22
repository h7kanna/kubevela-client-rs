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
pub struct V1PeriodConfigTemplateDetail {
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "schema")]
    pub schema: String,
    #[serde(rename = "scope")]
    pub scope: String,
    #[serde(rename = "sensitive")]
    pub sensitive: bool,
    #[serde(rename = "uiSchema")]
    pub ui_schema: Vec<crate::models::SchemaPeriodUiParameter>,
}

impl V1PeriodConfigTemplateDetail {
    pub fn new(alias: String, create_time: String, description: String, name: String, namespace: String, schema: String, scope: String, sensitive: bool, ui_schema: Vec<crate::models::SchemaPeriodUiParameter>) -> V1PeriodConfigTemplateDetail {
        V1PeriodConfigTemplateDetail {
            alias,
            create_time,
            description,
            name,
            namespace,
            schema,
            scope,
            sensitive,
            ui_schema,
        }
    }
}


