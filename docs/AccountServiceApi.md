# \AccountServiceApi

All URIs are relative to *http://api.madana.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_user**](AccountServiceApi.md#activate_user) | **get** /account/activation/{token} | 
[**create_password_reset**](AccountServiceApi.md#create_password_reset) | **post** /account/password | Sends an Password reset mail to the given MailAddress.
[**request_verification_mail**](AccountServiceApi.md#request_verification_mail) | **get** /account/verifymail | Used to request a new  activation-mail for the user.
[**update_password**](AccountServiceApi.md#update_password) | **put** /account/password | Receives the Password reset and tries to set the provided password for the user.



## activate_user

> std::path::PathBuf activate_user(token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_password_reset

> std::path::PathBuf create_password_reset(body)
Sends an Password reset mail to the given MailAddress.

Sends an Password reset mail to the given MailAddress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonMdnMailAddress**](JsonMdnMailAddress.md)> | - the MaiAddress under which the user has signed up |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_verification_mail

> ::std::collections::HashMap<String, serde_json::Value> request_verification_mail()
Used to request a new  activation-mail for the user.

Used to request a new  activation-mail for the user

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_password

> std::path::PathBuf update_password(body)
Receives the Password reset and tries to set the provided password for the user.

Receives the Password reset and tries to set the provided password for the user. The Password will only be set if a valid token is provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonMdnPasswordReset**](JsonMdnPasswordReset.md)> | - the MDN_PasswordReset Object |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

