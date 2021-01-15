/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.31
 * 
 * Generated by: https://openapi-generator.tech
 */

/// XmlNs0Environment : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XmlNs0Environment {
    /// 
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<String>>,
    #[serde(rename = "defaultRunConfiguration", skip_serializing_if = "Option::is_none")]
    pub default_run_configuration: Option<crate::models::XmlNs0RunConfig>,
    /// 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 
    #[serde(rename = "ipfsHash", skip_serializing_if = "Option::is_none")]
    pub ipfs_hash: Option<String>,
    /// 
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 
    #[serde(rename = "packages", skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    /// 
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// 
    #[serde(rename = "rootHashOffset", skip_serializing_if = "Option::is_none")]
    pub root_hash_offset: Option<String>,
    /// 
    #[serde(rename = "roothash", skip_serializing_if = "Option::is_none")]
    pub roothash: Option<String>,
    /// 
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl XmlNs0Environment {
    /// 
    pub fn new() -> XmlNs0Environment {
        XmlNs0Environment {
            content: None,
            default_run_configuration: None,
            description: None,
            ipfs_hash: None,
            name: None,
            packages: None,
            published: None,
            root_hash_offset: None,
            roothash: None,
            size: None,
            uuid: None,
        }
    }
}


