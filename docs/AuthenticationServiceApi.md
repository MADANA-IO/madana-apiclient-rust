# \AuthenticationServiceApi

All URIs are relative to *http://api.madana.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate_application**](AuthenticationServiceApi.md#authenticate_application) | **post** /authentication/application | Authenticates a new application and returns the token.
[**authenticate_ethereum_wallet**](AuthenticationServiceApi.md#authenticate_ethereum_wallet) | **post** /authentication/ethereum/{wallet} | 
[**authenticate_user**](AuthenticationServiceApi.md#authenticate_user) | **post** /authentication | Authenticates a new user and returns the token (  forbidden if the credentials cannot be validated ).
[**authenticate_with_ethereum_challenge**](AuthenticationServiceApi.md#authenticate_with_ethereum_challenge) | **post** /authentication/ethereum/{wallet}/challenge | 
[**get_fractal_authentication_url**](AuthenticationServiceApi.md#get_fractal_authentication_url) | **get** /authentication/fractal | Returns the AUthorization URL to verify a Twitter Accounts.
[**get_nonce_for_ethereum_wallet**](AuthenticationServiceApi.md#get_nonce_for_ethereum_wallet) | **get** /authentication/ethereum/{wallet} | Returns a nonce for the client which is used as content for the to be created signature.
[**get_object**](AuthenticationServiceApi.md#get_object) | **get** /authentication | Used to validate the active connection with the API.
[**get_twitter_authentication_url**](AuthenticationServiceApi.md#get_twitter_authentication_url) | **get** /authentication/twitter | Returns the AUthorization URL to verify a Twitter Accounts.
[**set_facebook_uid**](AuthenticationServiceApi.md#set_facebook_uid) | **post** /authentication/facebook | Used as Callback URL when users have successfully authorized their facbeook account.
[**set_fractal_uid**](AuthenticationServiceApi.md#set_fractal_uid) | **post** /authentication/fractal | 
[**set_twitter_uid**](AuthenticationServiceApi.md#set_twitter_uid) | **post** /authentication/twitter | 



## authenticate_application

> crate::models::JsonMdnToken authenticate_application(body)
Authenticates a new application and returns the token.

Authenticates a new application and returns the token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonMdnCertificate**](JsonMdnCertificate.md)> | the credentials used to validate the user |  |

### Return type

[**crate::models::JsonMdnToken**](json_MDN_Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_ethereum_wallet

> std::path::PathBuf authenticate_ethereum_wallet(wallet, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet** | **String** | the wallet which should be authenticated | [required] |
**body** | Option<[**JsonMdnOAuthToken**](JsonMdnOAuthToken.md)> | Token containing nonce and signate |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_user

> crate::models::JsonMdnToken authenticate_user(body)
Authenticates a new user and returns the token (  forbidden if the credentials cannot be validated ).

Authenticates a new user and returns the token (  forbidden if the credentials cannot be validated )

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonMdnUserCredentials**](JsonMdnUserCredentials.md)> | the credentials used to validate the user |  |

### Return type

[**crate::models::JsonMdnToken**](json_MDN_Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_with_ethereum_challenge

> std::path::PathBuf authenticate_with_ethereum_challenge(wallet, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet** | **String** | the wallet which should be authenticated | [required] |
**body** | Option<[**JsonMdnOAuthToken**](JsonMdnOAuthToken.md)> | Token containing nonce and signate |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fractal_authentication_url

> std::path::PathBuf get_fractal_authentication_url()
Returns the AUthorization URL to verify a Twitter Accounts.

Returns the AUthorization URL to verify a Twitter Accounts

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


## get_nonce_for_ethereum_wallet

> crate::models::JsonMdnToken get_nonce_for_ethereum_wallet(wallet, authorization)
Returns a nonce for the client which is used as content for the to be created signature.

Returns a nonce for the client which is used as content for the to be created signature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wallet** | **String** | - wallet address as String * @HTTP 417 If the address is not valid | [required] |
**authorization** | Option<**String**> | Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c |  |

### Return type

[**crate::models::JsonMdnToken**](json_MDN_Token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object

> ::std::collections::HashMap<String, serde_json::Value> get_object()
Used to validate the active connection with the API.

Used to validate the active connection with the API

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


## get_twitter_authentication_url

> std::path::PathBuf get_twitter_authentication_url()
Returns the AUthorization URL to verify a Twitter Accounts.

Returns the AUthorization URL to verify a Twitter Accounts

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


## set_facebook_uid

> std::path::PathBuf set_facebook_uid(body)
Used as Callback URL when users have successfully authorized their facbeook account.

Used as Callback URL when users have successfully authorized their facbeook account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_fractal_uid

> std::path::PathBuf set_fractal_uid(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_twitter_uid

> std::path::PathBuf set_twitter_uid(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JsonMdnOAuthToken**](JsonMdnOAuthToken.md)> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

