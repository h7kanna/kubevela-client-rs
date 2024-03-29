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
pub struct TypesPeriodInfo {
    #[serde(rename = "author")]
    pub author: Box<crate::models::TypesPeriodInfoLink>,
    #[serde(rename = "build")]
    pub build: Box<crate::models::TypesPeriodBuildInfo>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "links")]
    pub links: Vec<crate::models::TypesPeriodInfoLink>,
    #[serde(rename = "logos")]
    pub logos: Box<crate::models::TypesPeriodLogos>,
    #[serde(rename = "screenshots")]
    pub screenshots: Vec<crate::models::TypesPeriodScreenshots>,
    #[serde(rename = "updated")]
    pub updated: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl TypesPeriodInfo {
    pub fn new(author: crate::models::TypesPeriodInfoLink, build: crate::models::TypesPeriodBuildInfo, description: String, links: Vec<crate::models::TypesPeriodInfoLink>, logos: crate::models::TypesPeriodLogos, screenshots: Vec<crate::models::TypesPeriodScreenshots>, updated: String, version: String) -> TypesPeriodInfo {
        TypesPeriodInfo {
            author: Box::new(author),
            build: Box::new(build),
            description,
            links,
            logos: Box::new(logos),
            screenshots,
            updated,
            version,
        }
    }
}


