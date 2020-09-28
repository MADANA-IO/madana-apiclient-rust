# \CertificateServiceApi

All URIs are relative to *http://api.madana.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate_certificate**](CertificateServiceApi.md#authenticate_certificate) | **post** /certificates | Issues certificates for logged-in users.
[**get_certificate_by_fingerprint**](CertificateServiceApi.md#get_certificate_by_fingerprint) | **get** /certificates/{fingerprint} | 
[**get_root_certificate**](CertificateServiceApi.md#get_root_certificate) | **get** /certificates/root | 



## authenticate_certificate

> crate::models::JsonMdnCertificate authenticate_certificate(body)
Issues certificates for logged-in users.

Issues certificates for logged-in users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonMdnData**](JsonMdnData.md)> |  |  |

### Return type

[**crate::models::JsonMdnCertificate**](json_MDN_Certificate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_by_fingerprint

> std::path::PathBuf get_certificate_by_fingerprint(fingerprint)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fingerprint** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_root_certificate

> std::path::PathBuf get_root_certificate()


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

