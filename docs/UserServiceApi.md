# \UserServiceApi

All URIs are relative to *http://api.madana.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_object**](UserServiceApi.md#create_object) | **post** /users | Creates a new user object.
[**delete_object**](UserServiceApi.md#delete_object) | **delete** /users/{username} | Deletes an User based on the provided id and securitycontext.
[**delete_object_0**](UserServiceApi.md#delete_object_0) | **delete** /users/{username}/social/{platform}/{ident} | Deletes linked account from the user and securitycontext.
[**get_avatars**](UserServiceApi.md#get_avatars) | **get** /users/{username}/avatars | 
[**get_certificates**](UserServiceApi.md#get_certificates) | **get** /users/{username}/certificates | 
[**get_enclave_history**](UserServiceApi.md#get_enclave_history) | **get** /users/{username}/enclavehistory | 
[**get_object2**](UserServiceApi.md#get_object2) | **get** /users/{username} | 
[**set_avatar**](UserServiceApi.md#set_avatar) | **post** /users/{username}/avatars | 
[**set_settings**](UserServiceApi.md#set_settings) | **post** /users/{username}/settings | 
[**update_object**](UserServiceApi.md#update_object) | **put** /users/{username} | Updates Userproperties based on the provided user object.



## create_object

> std::path::PathBuf create_object(referrer, body)
Creates a new user object.

Creates a new user object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**referrer** | Option<**String**> |  |  |
**body** | Option<[**JsonMdnUser**](JsonMdnUser.md)> | provided user object inheriting properties and credentials |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object

> std::path::PathBuf delete_object(username)
Deletes an User based on the provided id and securitycontext.

Deletes an User based on the provided id and securitycontext

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_0

> std::path::PathBuf delete_object_0(ident, platform, username)
Deletes linked account from the user and securitycontext.

Deletes linked account from the user and securitycontext

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ident** | **String** |  | [required] |
**platform** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatars

> std::path::PathBuf get_avatars(username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificates

> std::path::PathBuf get_certificates(username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enclave_history

> std::path::PathBuf get_enclave_history(username, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**limit** | Option<**String**> |  |  |[default to 30]
**offset** | Option<**String**> |  |  |[default to 0]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object2

> std::path::PathBuf get_object2(username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_avatar

> std::path::PathBuf set_avatar(username, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**body** | Option<[**JsonMdnUserProfileImage**](JsonMdnUserProfileImage.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_settings

> std::path::PathBuf set_settings(username, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**body** | Option<[**JsonMdnUserSetting**](JsonMdnUserSetting.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_object

> std::path::PathBuf update_object(username, body)
Updates Userproperties based on the provided user object.

Updates Userproperties based on the provided user object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**body** | Option<[**JsonMdnUser**](JsonMdnUser.md)> | the new user object inherting all properties that should be change |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

