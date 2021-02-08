/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.50
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonNodeInfo : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonNodeInfo {
    /// 
    #[serde(rename = "cpuModel", skip_serializing_if = "Option::is_none")]
    pub cpu_model: Option<String>,
    /// 
    #[serde(rename = "operatingSystemUptime", skip_serializing_if = "Option::is_none")]
    pub operating_system_uptime: Option<f32>,
    /// 
    #[serde(rename = "cpuLogicalCount", skip_serializing_if = "Option::is_none")]
    pub cpu_logical_count: Option<i32>,
    /// 
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 
    #[serde(rename = "operatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// 
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// 
    #[serde(rename = "hardwareFirmware", skip_serializing_if = "Option::is_none")]
    pub hardware_firmware: Option<String>,
    /// 
    #[serde(rename = "cpuFrequency", skip_serializing_if = "Option::is_none")]
    pub cpu_frequency: Option<String>,
    /// 
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<String>>,
    /// 
    #[serde(rename = "cpuFamily", skip_serializing_if = "Option::is_none")]
    pub cpu_family: Option<String>,
    #[serde(rename = "sgxInfo", skip_serializing_if = "Option::is_none")]
    pub sgx_info: Option<crate::models::JsonSgxInfo>,
    /// 
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "ipfsInfo", skip_serializing_if = "Option::is_none")]
    pub ipfs_info: Option<crate::models::JsonIpfsSystemInfo>,
    /// 
    #[serde(rename = "cpuPhysicalCores", skip_serializing_if = "Option::is_none")]
    pub cpu_physical_cores: Option<i32>,
    /// 
    #[serde(rename = "connectionURL", skip_serializing_if = "Option::is_none")]
    pub connection_url: Option<String>,
    /// 
    #[serde(rename = "hardwareBaseboard", skip_serializing_if = "Option::is_none")]
    pub hardware_baseboard: Option<String>,
}

impl JsonNodeInfo {
    /// 
    pub fn new() -> JsonNodeInfo {
        JsonNodeInfo {
            cpu_model: None,
            operating_system_uptime: None,
            cpu_logical_count: None,
            owner: None,
            operating_system: None,
            status: None,
            memory: None,
            hardware_firmware: None,
            cpu_frequency: None,
            processors: None,
            cpu_family: None,
            sgx_info: None,
            public_key: None,
            ipfs_info: None,
            cpu_physical_cores: None,
            connection_url: None,
            hardware_baseboard: None,
        }
    }
}


