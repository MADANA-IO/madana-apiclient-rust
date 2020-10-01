/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.15-master.8
 * 
 * Generated by: https://openapi-generator.tech
 */

/// XmlNs0EnclaveProcess : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XmlNs0EnclaveProcess {
    /// 
    #[serde(rename = "attestationServer", skip_serializing_if = "Option::is_none")]
    pub attestation_server: Option<String>,
    /// 
    #[serde(rename = "consoleOutput", skip_serializing_if = "Option::is_none")]
    pub console_output: Option<String>,
    /// 
    #[serde(rename = "enclaveIdent", skip_serializing_if = "Option::is_none")]
    pub enclave_ident: Option<String>,
    #[serde(rename = "enclaveInputstream", skip_serializing_if = "Option::is_none")]
    pub enclave_inputstream: Option<crate::models::XmlNs0InputStream>,
    /// 
    #[serde(rename = "endingTime", skip_serializing_if = "Option::is_none")]
    pub ending_time: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<crate::models::XmlNs0Environment>,
    /// 
    #[serde(rename = "internalAttesationServer", skip_serializing_if = "Option::is_none")]
    pub internal_attesation_server: Option<String>,
    /// 
    #[serde(rename = "internalIdent", skip_serializing_if = "Option::is_none")]
    pub internal_ident: Option<String>,
    /// 
    #[serde(rename = "internalRemoteControlServer", skip_serializing_if = "Option::is_none")]
    pub internal_remote_control_server: Option<String>,
    /// 
    #[serde(rename = "internalWireguardServer", skip_serializing_if = "Option::is_none")]
    pub internal_wireguard_server: Option<String>,
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: Option<crate::models::XmlNs0Process>,
    /// 
    #[serde(rename = "publicIdent", skip_serializing_if = "Option::is_none")]
    pub public_ident: Option<String>,
    /// 
    #[serde(rename = "remoteControlServer", skip_serializing_if = "Option::is_none")]
    pub remote_control_server: Option<String>,
    /// 
    #[serde(rename = "signerIdent", skip_serializing_if = "Option::is_none")]
    pub signer_ident: Option<String>,
    /// 
    #[serde(rename = "startupCMD", skip_serializing_if = "Option::is_none")]
    pub startup_cmd: Option<String>,
    /// 
    #[serde(rename = "startupTime", skip_serializing_if = "Option::is_none")]
    pub startup_time: Option<String>,
    /// 
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "wgInterface", skip_serializing_if = "Option::is_none")]
    pub wg_interface: Option<crate::models::XmlNs0WireguardInterface>,
    /// 
    #[serde(rename = "wireguardPublicKey", skip_serializing_if = "Option::is_none")]
    pub wireguard_public_key: Option<String>,
    /// 
    #[serde(rename = "wireguardServer", skip_serializing_if = "Option::is_none")]
    pub wireguard_server: Option<String>,
}

impl XmlNs0EnclaveProcess {
    /// 
    pub fn new() -> XmlNs0EnclaveProcess {
        XmlNs0EnclaveProcess {
            attestation_server: None,
            console_output: None,
            enclave_ident: None,
            enclave_inputstream: None,
            ending_time: None,
            environment: None,
            internal_attesation_server: None,
            internal_ident: None,
            internal_remote_control_server: None,
            internal_wireguard_server: None,
            process: None,
            public_ident: None,
            remote_control_server: None,
            signer_ident: None,
            startup_cmd: None,
            startup_time: None,
            status: None,
            wg_interface: None,
            wireguard_public_key: None,
            wireguard_server: None,
        }
    }
}


