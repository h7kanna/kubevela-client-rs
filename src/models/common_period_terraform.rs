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
pub struct CommonPeriodTerraform {
    #[serde(rename = "configuration")]
    pub configuration: String,
    #[serde(rename = "customRegion", skip_serializing_if = "Option::is_none")]
    pub custom_region: Option<String>,
    #[serde(rename = "deleteResource", skip_serializing_if = "Option::is_none")]
    pub delete_resource: Option<bool>,
    #[serde(rename = "gitCredentialsSecretReference", skip_serializing_if = "Option::is_none")]
    pub git_credentials_secret_reference: Option<Box<crate::models::V1PeriodSecretReference>>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "providerRef", skip_serializing_if = "Option::is_none")]
    pub provider_ref: Option<Box<crate::models::TypesPeriodReference>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "writeConnectionSecretToRef", skip_serializing_if = "Option::is_none")]
    pub write_connection_secret_to_ref: Option<Box<crate::models::TypesPeriodSecretReference>>,
}

impl CommonPeriodTerraform {
    pub fn new(configuration: String) -> CommonPeriodTerraform {
        CommonPeriodTerraform {
            configuration,
            custom_region: None,
            delete_resource: None,
            git_credentials_secret_reference: None,
            path: None,
            provider_ref: None,
            r#type: None,
            write_connection_secret_to_ref: None,
        }
    }
}


