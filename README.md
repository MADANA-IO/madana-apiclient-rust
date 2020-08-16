# Rust API client for madanasampleclientrust

<h1>Using the madana-api</h1>
       <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available 
       endpoints and used datamodels.   </p>    

 <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p> 
 <br>
  <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>    
  <br>

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.4.14-master.16
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://api.madana.io/rest*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountServiceApi* | [**activate_user**](docs/AccountServiceApi.md#activate_user) | **Get** /account/activation/{token} | 
*AccountServiceApi* | [**create_object**](docs/AccountServiceApi.md#create_object) | **Post** /account/password | Sends an Password reset mail to the given MailAddress.
*AccountServiceApi* | [**request_verification_mail**](docs/AccountServiceApi.md#request_verification_mail) | **Get** /account/verifymail | Used to request a new  activation-mail for the user.
*AccountServiceApi* | [**update_object**](docs/AccountServiceApi.md#update_object) | **Put** /account/password | Receives the Password reset and tries to set the provided password for the user.
*AuthenticationServiceApi* | [**authenticate_application**](docs/AuthenticationServiceApi.md#authenticate_application) | **Post** /authentication/application | Authenticates a new application and returns the token.
*AuthenticationServiceApi* | [**authenticate_ethereum_wallet**](docs/AuthenticationServiceApi.md#authenticate_ethereum_wallet) | **Post** /authentication/ethereum/{wallet} | 
*AuthenticationServiceApi* | [**authenticate_user**](docs/AuthenticationServiceApi.md#authenticate_user) | **Post** /authentication | Authenticates a new user and returns the token (  forbidden if the credentials cannot be validated ).
*AuthenticationServiceApi* | [**authenticate_with_ethereum_challenge**](docs/AuthenticationServiceApi.md#authenticate_with_ethereum_challenge) | **Post** /authentication/ethereum/{wallet}/challenge | 
*AuthenticationServiceApi* | [**get_fractal_authentication_url**](docs/AuthenticationServiceApi.md#get_fractal_authentication_url) | **Get** /authentication/fractal | Returns the AUthorization URL to verify a Twitter Accounts.
*AuthenticationServiceApi* | [**get_nonce_for_ethereum_wallet**](docs/AuthenticationServiceApi.md#get_nonce_for_ethereum_wallet) | **Get** /authentication/ethereum/{wallet} | Returns a nonce for the client which is used as content for the to be created signature.
*AuthenticationServiceApi* | [**get_object**](docs/AuthenticationServiceApi.md#get_object) | **Get** /authentication | Used to validate the active connection with the API.
*AuthenticationServiceApi* | [**get_twitter_authentication_url**](docs/AuthenticationServiceApi.md#get_twitter_authentication_url) | **Get** /authentication/twitter | Returns the AUthorization URL to verify a Twitter Accounts.
*AuthenticationServiceApi* | [**set_facebook_uid**](docs/AuthenticationServiceApi.md#set_facebook_uid) | **Post** /authentication/facebook | Used as Callback URL when users have successfully authorized their facbeook account.
*AuthenticationServiceApi* | [**set_fractal_uid**](docs/AuthenticationServiceApi.md#set_fractal_uid) | **Post** /authentication/fractal | 
*AuthenticationServiceApi* | [**set_twitter_uid**](docs/AuthenticationServiceApi.md#set_twitter_uid) | **Post** /authentication/twitter | 
*CertificateServiceApi* | [**authenticate_certificate**](docs/CertificateServiceApi.md#authenticate_certificate) | **Post** /certificates | Issues certificates for logged-in users.
*CertificateServiceApi* | [**get_certificate**](docs/CertificateServiceApi.md#get_certificate) | **Get** /certificates/root | 
*CertificateServiceApi* | [**get_certificate_0**](docs/CertificateServiceApi.md#get_certificate_0) | **Get** /certificates/{fingerprint} | 
*DataCollectionServiceApi* | [**get_methods_for_type**](docs/DataCollectionServiceApi.md#get_methods_for_type) | **Get** /datacollection/types/{name}/methods | 
*DataCollectionServiceApi* | [**get_nodes**](docs/DataCollectionServiceApi.md#get_nodes) | **Get** /datacollection/methods | 
*DataCollectionServiceApi* | [**get_types**](docs/DataCollectionServiceApi.md#get_types) | **Get** /datacollection/types | 
*EnclaveServiceApi* | [**approve_enclave**](docs/EnclaveServiceApi.md#approve_enclave) | **Post** /enclaves/{uuid}/approval | 
*EnclaveServiceApi* | [**assign_enclave_agent**](docs/EnclaveServiceApi.md#assign_enclave_agent) | **Post** /enclaves/{uuid}/assign | 
*EnclaveServiceApi* | [**attestate_enclave**](docs/EnclaveServiceApi.md#attestate_enclave) | **Post** /enclaves/{uuid}/attestation | 
*EnclaveServiceApi* | [**create_enclave_run_request**](docs/EnclaveServiceApi.md#create_enclave_run_request) | **Post** /enclaves | 
*EnclaveServiceApi* | [**get_enclave**](docs/EnclaveServiceApi.md#get_enclave) | **Get** /enclaves/{uuid} | 
*EnclaveServiceApi* | [**get_enclave_types**](docs/EnclaveServiceApi.md#get_enclave_types) | **Get** /enclaves/types | 
*EnclaveServiceApi* | [**get_enclaves**](docs/EnclaveServiceApi.md#get_enclaves) | **Get** /enclaves | Returns UUIDs of existing analyses.
*EnclaveServiceApi* | [**kill_enclave**](docs/EnclaveServiceApi.md#kill_enclave) | **Post** /enclaves/{uuid}/kill | 
*EnvironmentServiceApi* | [**delete_environment**](docs/EnvironmentServiceApi.md#delete_environment) | **Delete** /environments/{uuid} | 
*EnvironmentServiceApi* | [**delete_environment_subscription**](docs/EnvironmentServiceApi.md#delete_environment_subscription) | **Delete** /environments/{uuid}/subscribe | 
*EnvironmentServiceApi* | [**get_all_requests**](docs/EnvironmentServiceApi.md#get_all_requests) | **Get** /environments | Returns UUIDs of existing analyses.
*EnvironmentServiceApi* | [**get_environment**](docs/EnvironmentServiceApi.md#get_environment) | **Get** /environments/{uuid} | 
*EnvironmentServiceApi* | [**get_published_environments**](docs/EnvironmentServiceApi.md#get_published_environments) | **Get** /environments/published | 
*EnvironmentServiceApi* | [**get_subscribed_environments**](docs/EnvironmentServiceApi.md#get_subscribed_environments) | **Get** /environments/subscriptions | 
*EnvironmentServiceApi* | [**publish_environment**](docs/EnvironmentServiceApi.md#publish_environment) | **Post** /environments | 
*EnvironmentServiceApi* | [**subscribe_environment**](docs/EnvironmentServiceApi.md#subscribe_environment) | **Post** /environments/{uuid}/subscribe | 
*EnvironmentServiceApi* | [**update_environment**](docs/EnvironmentServiceApi.md#update_environment) | **Put** /environments/{uuid} | 
*NodeServiceApi* | [**get_bootstrap**](docs/NodeServiceApi.md#get_bootstrap) | **Get** /nodes/bootstrap | 
*NodeServiceApi* | [**get_nodes2**](docs/NodeServiceApi.md#get_nodes2) | **Get** /nodes | 
*NodeServiceApi* | [**post_node_info**](docs/NodeServiceApi.md#post_node_info) | **Post** /nodes | 
*OrganizationServiceApi* | [**get_nodes3**](docs/OrganizationServiceApi.md#get_nodes3) | **Get** /organizations | 
*RequestServiceApi* | [**add_data**](docs/RequestServiceApi.md#add_data) | **Post** /requests/{uuid}/data | Is used to upload and park the data till the AnalysisRequest gets processed.
*RequestServiceApi* | [**cancel_processing**](docs/RequestServiceApi.md#cancel_processing) | **Post** /requests/{uuid}/cancel | Endpoint is called from the Analysis Processing entity to submit the result.
*RequestServiceApi* | [**create_new_request**](docs/RequestServiceApi.md#create_new_request) | **Post** /requests | Endpoint used to create a new Analysis Request.
*RequestServiceApi* | [**get_actions**](docs/RequestServiceApi.md#get_actions) | **Get** /requests/actions | 
*RequestServiceApi* | [**get_agent**](docs/RequestServiceApi.md#get_agent) | **Get** /requests/{uuid}/agent | Is called from the APE to request all parked datasets.
*RequestServiceApi* | [**get_all_requests2**](docs/RequestServiceApi.md#get_all_requests2) | **Get** /requests | Returns UUIDs of existing analyses.
*RequestServiceApi* | [**get_data**](docs/RequestServiceApi.md#get_data) | **Get** /requests/{uuid}/data | Is called from the APE to request all parked datasets.
*RequestServiceApi* | [**get_request**](docs/RequestServiceApi.md#get_request) | **Get** /requests/{uuid} | Returns the details for certain Request.
*RequestServiceApi* | [**get_result**](docs/RequestServiceApi.md#get_result) | **Get** /requests/{uuid}/result | Can be called from creator to request the AnalysisResult.
*RequestServiceApi* | [**get_status**](docs/RequestServiceApi.md#get_status) | **Get** /requests/stats | 
*RequestServiceApi* | [**give_consent**](docs/RequestServiceApi.md#give_consent) | **Post** /requests/{uuid}/consent | Used to give consent for request.
*RequestServiceApi* | [**init_request_parameters**](docs/RequestServiceApi.md#init_request_parameters) | **Post** /requests/{uuid} | Endpoint used initialized addition datacollection parameters for requester.
*RequestServiceApi* | [**set_agent**](docs/RequestServiceApi.md#set_agent) | **Post** /requests/{uuid}/agent | Is called from the APE to request all parked datasets.
*RequestServiceApi* | [**set_result**](docs/RequestServiceApi.md#set_result) | **Post** /requests/{uuid}/result | Endpoint is called from the Analysis Processing entity to submit the result.
*SocialPlatformServiceApi* | [**get_platforms**](docs/SocialPlatformServiceApi.md#get_platforms) | **Get** /platforms | Used to Handle Incoming Webhooks from Facebook.
*SocialPlatformServiceApi* | [**listen_twitter_webhook**](docs/SocialPlatformServiceApi.md#listen_twitter_webhook) | **Post** /platforms/twitter | Used to Handle Incoming Webhooks from Facebook.
*SocialPlatformServiceApi* | [**register_twitter_webhook**](docs/SocialPlatformServiceApi.md#register_twitter_webhook) | **Get** /platforms/twitter | Used to Handle Incoming Webhooks from Twitter.
*SocialServiceApi* | [**get_my_profile**](docs/SocialServiceApi.md#get_my_profile) | **Get** /social/profiles/me | 
*SocialServiceApi* | [**get_platforms2**](docs/SocialServiceApi.md#get_platforms2) | **Get** /social | Returns all Platforms / Systems that can be Connected to the MADANA Service.
*SocialServiceApi* | [**get_ranking**](docs/SocialServiceApi.md#get_ranking) | **Get** /social/ranking | Returns the Ranking by PTS within the System.
*SocialServiceApi* | [**get_social_platform_feed**](docs/SocialServiceApi.md#get_social_platform_feed) | **Get** /social/feed/{platform} | 
*SocialServiceApi* | [**get_user_profile**](docs/SocialServiceApi.md#get_user_profile) | **Get** /social/profiles/{username} | 
*SocialServiceApi* | [**get_user_profile_0**](docs/SocialServiceApi.md#get_user_profile_0) | **Get** /social/profiles/{username}/simple | 
*SystemServiceApi* | [**get_all_objects**](docs/SystemServiceApi.md#get_all_objects) | **Get** /system/health | 
*SystemServiceApi* | [**get_application**](docs/SystemServiceApi.md#get_application) | **Get** /system/usage | Return the current application usage.
*UserServiceApi* | [**create_object2**](docs/UserServiceApi.md#create_object2) | **Post** /users | Creates a new user object.
*UserServiceApi* | [**delete_object**](docs/UserServiceApi.md#delete_object) | **Delete** /users/{username} | Deletes an User based on the provided id and securitycontext.
*UserServiceApi* | [**delete_object_0**](docs/UserServiceApi.md#delete_object_0) | **Delete** /users/{username}/social/{platform}/{ident} | Deletes linked account from the user and securitycontext.
*UserServiceApi* | [**get_avatars**](docs/UserServiceApi.md#get_avatars) | **Get** /users/{username}/avatars | 
*UserServiceApi* | [**get_certificates**](docs/UserServiceApi.md#get_certificates) | **Get** /users/{username}/certificates | 
*UserServiceApi* | [**get_object2**](docs/UserServiceApi.md#get_object2) | **Get** /users/{username} | 
*UserServiceApi* | [**set_avatar**](docs/UserServiceApi.md#set_avatar) | **Post** /users/{username}/avatars | 
*UserServiceApi* | [**set_settings**](docs/UserServiceApi.md#set_settings) | **Post** /users/{username}/settings | 
*UserServiceApi* | [**update_object2**](docs/UserServiceApi.md#update_object2) | **Put** /users/{username} | Updates Userproperties based on the provided user object.


## Documentation For Models

 - [JsonDiskConfig](docs/JsonDiskConfig.md)
 - [JsonEnclaveProcess](docs/JsonEnclaveProcess.md)
 - [JsonEnclaveRunRequest](docs/JsonEnclaveRunRequest.md)
 - [JsonEnclaveRunningAttestation](docs/JsonEnclaveRunningAttestation.md)
 - [JsonEnclaveRunningAttestationApproval](docs/JsonEnclaveRunningAttestationApproval.md)
 - [JsonEnclaveRunningAttestationApprovalAllOf](docs/JsonEnclaveRunningAttestationApprovalAllOf.md)
 - [JsonEnvironment](docs/JsonEnvironment.md)
 - [JsonEnvironmentPublishingRequest](docs/JsonEnvironmentPublishingRequest.md)
 - [JsonIpfsSystemInfo](docs/JsonIpfsSystemInfo.md)
 - [JsonMdnAUserObject](docs/JsonMdnAUserObject.md)
 - [JsonMdnCertificate](docs/JsonMdnCertificate.md)
 - [JsonMdnData](docs/JsonMdnData.md)
 - [JsonMdnMailAddress](docs/JsonMdnMailAddress.md)
 - [JsonMdnOAuthToken](docs/JsonMdnOAuthToken.md)
 - [JsonMdnPasswordReset](docs/JsonMdnPasswordReset.md)
 - [JsonMdnSetting](docs/JsonMdnSetting.md)
 - [JsonMdnSocialUserObject](docs/JsonMdnSocialUserObject.md)
 - [JsonMdnToken](docs/JsonMdnToken.md)
 - [JsonMdnUser](docs/JsonMdnUser.md)
 - [JsonMdnUserAllOf](docs/JsonMdnUserAllOf.md)
 - [JsonMdnUserCredentials](docs/JsonMdnUserCredentials.md)
 - [JsonMdnUserProfileImage](docs/JsonMdnUserProfileImage.md)
 - [JsonMdnUserSetting](docs/JsonMdnUserSetting.md)
 - [JsonMdnUserSettingAllOf](docs/JsonMdnUserSettingAllOf.md)
 - [JsonNetworkInterface](docs/JsonNetworkInterface.md)
 - [JsonNodeInfo](docs/JsonNodeInfo.md)
 - [JsonProcess](docs/JsonProcess.md)
 - [JsonRunConfig](docs/JsonRunConfig.md)
 - [JsonSignedData](docs/JsonSignedData.md)
 - [JsonWireguardInterface](docs/JsonWireguardInterface.md)
 - [JsonWireguardInterfaceAllOf](docs/JsonWireguardInterfaceAllOf.md)
 - [XmlNs0DiskConfig](docs/XmlNs0DiskConfig.md)
 - [XmlNs0DiskConfigAllOf](docs/XmlNs0DiskConfigAllOf.md)
 - [XmlNs0EnclaveProcess](docs/XmlNs0EnclaveProcess.md)
 - [XmlNs0EnclaveProcessAllOf](docs/XmlNs0EnclaveProcessAllOf.md)
 - [XmlNs0EnclaveRunningAttestation](docs/XmlNs0EnclaveRunningAttestation.md)
 - [XmlNs0EnclaveRunningAttestationAllOf](docs/XmlNs0EnclaveRunningAttestationAllOf.md)
 - [XmlNs0EnclaveRunningAttestationApproval](docs/XmlNs0EnclaveRunningAttestationApproval.md)
 - [XmlNs0EnclaveRunningAttestationApprovalAllOf](docs/XmlNs0EnclaveRunningAttestationApprovalAllOf.md)
 - [XmlNs0Environment](docs/XmlNs0Environment.md)
 - [XmlNs0EnvironmentAllOf](docs/XmlNs0EnvironmentAllOf.md)
 - [XmlNs0IpfsSystemInfo](docs/XmlNs0IpfsSystemInfo.md)
 - [XmlNs0IpfsSystemInfoAllOf](docs/XmlNs0IpfsSystemInfoAllOf.md)
 - [XmlNs0MdnSetting](docs/XmlNs0MdnSetting.md)
 - [XmlNs0MdnSettingAllOf](docs/XmlNs0MdnSettingAllOf.md)
 - [XmlNs0MdnUserProfileImage](docs/XmlNs0MdnUserProfileImage.md)
 - [XmlNs0MdnUserProfileImageAllOf](docs/XmlNs0MdnUserProfileImageAllOf.md)
 - [XmlNs0MdnUserSetting](docs/XmlNs0MdnUserSetting.md)
 - [XmlNs0MdnUserSettingAllOf](docs/XmlNs0MdnUserSettingAllOf.md)
 - [XmlNs0NetworkInterface](docs/XmlNs0NetworkInterface.md)
 - [XmlNs0NetworkInterfaceAllOf](docs/XmlNs0NetworkInterfaceAllOf.md)
 - [XmlNs0NodeInfo](docs/XmlNs0NodeInfo.md)
 - [XmlNs0NodeInfoAllOf](docs/XmlNs0NodeInfoAllOf.md)
 - [XmlNs0Process](docs/XmlNs0Process.md)
 - [XmlNs0RunConfig](docs/XmlNs0RunConfig.md)
 - [XmlNs0RunConfigAllOf](docs/XmlNs0RunConfigAllOf.md)
 - [XmlNs0SignedData](docs/XmlNs0SignedData.md)
 - [XmlNs0SignedDataAllOf](docs/XmlNs0SignedDataAllOf.md)
 - [XmlNs0WireguardInterface](docs/XmlNs0WireguardInterface.md)
 - [XmlNs0WireguardInterfaceAllOf](docs/XmlNs0WireguardInterfaceAllOf.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



