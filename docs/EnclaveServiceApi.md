# \EnclaveServiceApi

All URIs are relative to *http://api.madana.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_history**](EnclaveServiceApi.md#add_history) | **Post** /enclaves/{uuid}/history | 
[**approve_enclave**](EnclaveServiceApi.md#approve_enclave) | **Post** /enclaves/{uuid}/approval | 
[**assign_enclave_agent**](EnclaveServiceApi.md#assign_enclave_agent) | **Post** /enclaves/{uuid}/assign | 
[**attestate_enclave**](EnclaveServiceApi.md#attestate_enclave) | **Post** /enclaves/{uuid}/attestation | 
[**create_enclave_run_request**](EnclaveServiceApi.md#create_enclave_run_request) | **Post** /enclaves | 
[**get_enclave**](EnclaveServiceApi.md#get_enclave) | **Get** /enclaves/{uuid} | 
[**get_enclave_types**](EnclaveServiceApi.md#get_enclave_types) | **Get** /enclaves/types | 
[**get_enclaves**](EnclaveServiceApi.md#get_enclaves) | **Get** /enclaves | Returns UUIDs of existing analyses.
[**kill_enclave**](EnclaveServiceApi.md#kill_enclave) | **Post** /enclaves/{uuid}/kill | 



## add_history

> std::path::PathBuf add_history(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | Option<[**JsonSignedData**](JsonSignedData.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_enclave

> std::path::PathBuf approve_enclave(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | Option<[**JsonEnclaveRunningAttestationApproval**](JsonEnclaveRunningAttestationApproval.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_enclave_agent

> std::path::PathBuf assign_enclave_agent(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | Option<[**JsonNodeInfo**](JsonNodeInfo.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attestate_enclave

> std::path::PathBuf attestate_enclave(uuid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**body** | Option<[**JsonEnclaveRunningAttestation**](JsonEnclaveRunningAttestation.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_enclave_run_request

> std::path::PathBuf create_enclave_run_request(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonEnclaveRunRequest**](JsonEnclaveRunRequest.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enclave

> std::path::PathBuf get_enclave(uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enclave_types

> std::path::PathBuf get_enclave_types()


### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enclaves

> std::path::PathBuf get_enclaves(authorization, created, limit, offset, status)
Returns UUIDs of existing analyses.

Returns UUIDs of existing analyses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<**String**> | Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c |  |
**created** | Option<**String**> | - if Queryparam \"created=true\" only the UUIDs of own Requests are shown |  |[default to true]
**limit** | Option<**String**> | Used for offset pagination. Limit/Offset Paging would look like GET /request?limit=20&offset=100. This query would return the 20 rows starting with the 100th row |  |[default to 30]
**offset** | Option<**String**> | Used for offset pagination. Limit/Offset Paging would look like GET /request?limit=20&offset=100. This query would return the 20 rows starting with the 100th row |  |[default to 0]
**status** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kill_enclave

> std::path::PathBuf kill_enclave(uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

