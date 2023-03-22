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
pub struct CloudproviderPeriodCloudCluster {
    #[serde(rename = "apiServerURL")]
    pub api_server_url: String,
    #[serde(rename = "dashboardURL")]
    pub dashboard_url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "regionID")]
    pub region_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "vpcID")]
    pub vpc_id: String,
    #[serde(rename = "zone")]
    pub zone: String,
    #[serde(rename = "zoneID")]
    pub zone_id: String,
}

impl CloudproviderPeriodCloudCluster {
    pub fn new(api_server_url: String, dashboard_url: String, id: String, labels: ::std::collections::HashMap<String, String>, name: String, provider: String, region_id: String, status: String, r#type: String, vpc_id: String, zone: String, zone_id: String) -> CloudproviderPeriodCloudCluster {
        CloudproviderPeriodCloudCluster {
            api_server_url,
            dashboard_url,
            id,
            labels,
            name,
            provider,
            region_id,
            status,
            r#type,
            vpc_id,
            zone,
            zone_id,
        }
    }
}


