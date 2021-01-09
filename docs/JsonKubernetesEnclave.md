# JsonKubernetesEnclave

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wg_interface** | Option<[**crate::models::JsonWireguardInterface**](json_WireguardInterface.md)> |  | [optional]
**signer_ident** | Option<**String**> |  | [optional]
**environment** | Option<[**crate::models::JsonEnvironment**](json_Environment.md)> |  | [optional]
**kubernetes_enclave** | Option<[**crate::models::JsonKubernetesEnclave**](json_KubernetesEnclave.md)> |  | [optional]
**internal_wireguard_server** | Option<**String**> |  | [optional]
**public_ident** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**attestation_server** | Option<**String**> |  | [optional]
**enclave_inputstream** | Option<[**serde_json::Value**](.md)> |  | [optional]
**internal_remote_control_server** | Option<**String**> |  | [optional]
**startup_cmd** | Option<**String**> |  | [optional]
**port_mapping** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**startup_time** | Option<**String**> |  | [optional]
**wireguard_server** | Option<**String**> |  | [optional]
**internal_ident** | Option<**String**> |  | [optional]
**enclave_ident** | Option<**String**> |  | [optional]
**internal_attesation_server** | Option<**String**> |  | [optional]
**console_output** | Option<**String**> |  | [optional]
**ending_time** | Option<**String**> |  | [optional]
**wireguard_public_key** | Option<**String**> |  | [optional]
**process** | Option<[**crate::models::JsonProcess**](json_Process.md)> |  | [optional]
**remote_control_server** | Option<**String**> |  | [optional]
**ports** | Option<[**Vec<crate::models::JsonEnclavePort>**](json_EnclavePort.md)> |  | [optional]
**remote_control_ip** | Option<**String**> |  | [optional]
**debug_info** | Option<**String**> |  | [optional]
**enclave_replica_set_events** | Option<[**crate::models::JsonV1EventList**](json_V1EventList.md)> |  | [optional]
**attestation_port** | Option<**i32**> |  | [optional]
**is_using_init_container** | Option<**bool**> |  | [optional]
**wireguard_port** | Option<**i32**> |  | [optional]
**enclave_deployment_events** | Option<[**crate::models::JsonV1EventList**](json_V1EventList.md)> |  | [optional]
**enclave_pod_events** | Option<[**crate::models::JsonV1EventList**](json_V1EventList.md)> |  | [optional]
**pod_phase** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


