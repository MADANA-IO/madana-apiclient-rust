/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.14-master.16
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonMdnAUserObject : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonMdnAUserObject {
    /// 
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 
    #[serde(rename = "activated", skip_serializing_if = "Option::is_none")]
    pub activated: Option<String>,
    /// 
    #[serde(rename = "lastActive", skip_serializing_if = "Option::is_none")]
    pub last_active: Option<String>,
    /// 
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl JsonMdnAUserObject {
    /// 
    pub fn new() -> JsonMdnAUserObject {
        JsonMdnAUserObject {
            created: None,
            activated: None,
            last_active: None,
            image: None,
            user_name: None,
        }
    }
}


