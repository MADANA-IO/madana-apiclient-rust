/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.54
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonEnclaveRunningAttestation : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonEnclaveRunningAttestation {
    #[serde(rename = "nodeInfo", skip_serializing_if = "Option::is_none")]
    pub node_info: Option<crate::models::JsonNodeInfo>,
    #[serde(rename = "enclaveProcess", skip_serializing_if = "Option::is_none")]
    pub enclave_process: Option<crate::models::JsonEnclaveProcess>,
}

impl JsonEnclaveRunningAttestation {
    /// 
    pub fn new() -> JsonEnclaveRunningAttestation {
        JsonEnclaveRunningAttestation {
            node_info: None,
            enclave_process: None,
        }
    }
}


