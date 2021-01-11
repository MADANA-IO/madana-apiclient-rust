# JsonEnclaveProcess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**process** | Option<[**crate::models::JsonProcess**](json_Process.md)> |  | [optional]
**enclave_ident** | Option<**String**> |  | [optional]
**enclave_inputstream** | Option<[**serde_json::Value**](.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**kubernetes_enclave** | Option<[**crate::models::JsonKubernetesEnclave**](json_KubernetesEnclave.md)> |  | [optional]
**internal_wireguard_server** | Option<**String**> |  | [optional]
**attestation_server** | Option<**String**> |  | [optional]
**remote_control_server** | Option<**String**> |  | [optional]
**internal_remote_control_server** | Option<**String**> |  | [optional]
**wg_interface** | Option<[**crate::models::JsonWireguardInterface**](json_WireguardInterface.md)> |  | [optional]
**console_output** | Option<**String**> |  | [optional]
**environment** | Option<[**crate::models::JsonEnvironment**](json_Environment.md)> |  | [optional]
**public_ident** | Option<**String**> |  | [optional]
**wireguard_public_key** | Option<**String**> |  | [optional]
**startup_cmd** | Option<**String**> |  | [optional]
**ports** | Option<[**Vec<crate::models::JsonEnclavePort>**](json_EnclavePort.md)> |  | [optional]
**signer_ident** | Option<**String**> |  | [optional]
**internal_ident** | Option<**String**> |  | [optional]
**internal_attesation_server** | Option<**String**> |  | [optional]
**startup_time** | Option<**String**> |  | [optional]
**port_mapping** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**wireguard_server** | Option<**String**> |  | [optional]
**ending_time** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


