# Rust API client for madana_apiclien

<h1>Using the madana-api</h1>
       <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available 
       endpoints and used datamodels.   </p>    

 <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p> 
 <br>
  <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>    
  <br>

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.5.0-master.55
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
*AccountServiceApi* | [**activate_user**](docs/AccountServiceApi.md#activate_user) | **get** /account/activation/{token} | 
*AccountServiceApi* | [**create_password_reset**](docs/AccountServiceApi.md#create_password_reset) | **post** /account/password | Sends an Password reset mail to the given MailAddress.
*AccountServiceApi* | [**request_verification_mail**](docs/AccountServiceApi.md#request_verification_mail) | **get** /account/verifymail | Used to request a new  activation-mail for the user.
*AccountServiceApi* | [**update_password**](docs/AccountServiceApi.md#update_password) | **put** /account/password | Receives the Password reset and tries to set the provided password for the user.
*AuthenticationServiceApi* | [**authenticate_application**](docs/AuthenticationServiceApi.md#authenticate_application) | **post** /authentication/application | Authenticates a new application and returns the token.
*AuthenticationServiceApi* | [**authenticate_ethereum_wallet**](docs/AuthenticationServiceApi.md#authenticate_ethereum_wallet) | **post** /authentication/ethereum/{wallet} | 
*AuthenticationServiceApi* | [**authenticate_user**](docs/AuthenticationServiceApi.md#authenticate_user) | **post** /authentication | Authenticates a new user and returns the token (  forbidden if the credentials cannot be validated ).
*AuthenticationServiceApi* | [**authenticate_with_ethereum_challenge**](docs/AuthenticationServiceApi.md#authenticate_with_ethereum_challenge) | **post** /authentication/ethereum/{wallet}/challenge | 
*AuthenticationServiceApi* | [**get_fractal_authentication_url**](docs/AuthenticationServiceApi.md#get_fractal_authentication_url) | **get** /authentication/fractal | Returns the AUthorization URL to verify a Twitter Accounts.
*AuthenticationServiceApi* | [**get_nonce_for_ethereum_wallet**](docs/AuthenticationServiceApi.md#get_nonce_for_ethereum_wallet) | **get** /authentication/ethereum/{wallet} | Returns a nonce for the client which is used as content for the to be created signature.
*AuthenticationServiceApi* | [**get_object**](docs/AuthenticationServiceApi.md#get_object) | **get** /authentication | Used to validate the active connection with the API.
*AuthenticationServiceApi* | [**get_twitter_authentication_url**](docs/AuthenticationServiceApi.md#get_twitter_authentication_url) | **get** /authentication/twitter | Returns the AUthorization URL to verify a Twitter Accounts.
*AuthenticationServiceApi* | [**set_facebook_uid**](docs/AuthenticationServiceApi.md#set_facebook_uid) | **post** /authentication/facebook | Used as Callback URL when users have successfully authorized their facbeook account.
*AuthenticationServiceApi* | [**set_fractal_uid**](docs/AuthenticationServiceApi.md#set_fractal_uid) | **post** /authentication/fractal | 
*AuthenticationServiceApi* | [**set_twitter_uid**](docs/AuthenticationServiceApi.md#set_twitter_uid) | **post** /authentication/twitter | 
*CertificateServiceApi* | [**authenticate_certificate**](docs/CertificateServiceApi.md#authenticate_certificate) | **post** /certificates | Issues certificates for logged-in users.
*CertificateServiceApi* | [**get_certificate_by_fingerprint**](docs/CertificateServiceApi.md#get_certificate_by_fingerprint) | **get** /certificates/{fingerprint} | 
*CertificateServiceApi* | [**get_root_certificate**](docs/CertificateServiceApi.md#get_root_certificate) | **get** /certificates/root | 
*DataCollectionServiceApi* | [**get_methods_for_type**](docs/DataCollectionServiceApi.md#get_methods_for_type) | **get** /datacollection/types/{name}/methods | 
*DataCollectionServiceApi* | [**get_nodes**](docs/DataCollectionServiceApi.md#get_nodes) | **get** /datacollection/methods | 
*DataCollectionServiceApi* | [**get_types**](docs/DataCollectionServiceApi.md#get_types) | **get** /datacollection/types | 
*EnclaveServiceApi* | [**add_history**](docs/EnclaveServiceApi.md#add_history) | **post** /enclaves/{uuid}/history | 
*EnclaveServiceApi* | [**approve_enclave**](docs/EnclaveServiceApi.md#approve_enclave) | **post** /enclaves/{uuid}/approval | 
*EnclaveServiceApi* | [**assign_enclave_agent**](docs/EnclaveServiceApi.md#assign_enclave_agent) | **post** /enclaves/{uuid}/assign | 
*EnclaveServiceApi* | [**attestate_enclave**](docs/EnclaveServiceApi.md#attestate_enclave) | **post** /enclaves/{uuid}/attestation | 
*EnclaveServiceApi* | [**create_enclave_run_request**](docs/EnclaveServiceApi.md#create_enclave_run_request) | **post** /enclaves | 
*EnclaveServiceApi* | [**get_enclave**](docs/EnclaveServiceApi.md#get_enclave) | **get** /enclaves/{uuid} | 
*EnclaveServiceApi* | [**get_enclave_types**](docs/EnclaveServiceApi.md#get_enclave_types) | **get** /enclaves/types | 
*EnclaveServiceApi* | [**get_enclaves**](docs/EnclaveServiceApi.md#get_enclaves) | **get** /enclaves | Returns UUIDs of existing analyses.
*EnclaveServiceApi* | [**get_stats**](docs/EnclaveServiceApi.md#get_stats) | **get** /enclaves/stats | 
*EnclaveServiceApi* | [**kill_enclave**](docs/EnclaveServiceApi.md#kill_enclave) | **post** /enclaves/{uuid}/kill | 
*EnvironmentServiceApi* | [**delete_environment**](docs/EnvironmentServiceApi.md#delete_environment) | **delete** /environments/{uuid} | 
*EnvironmentServiceApi* | [**delete_environment_subscription**](docs/EnvironmentServiceApi.md#delete_environment_subscription) | **delete** /environments/{uuid}/subscribe | 
*EnvironmentServiceApi* | [**get_environment**](docs/EnvironmentServiceApi.md#get_environment) | **get** /environments/{uuid} | 
*EnvironmentServiceApi* | [**get_environments**](docs/EnvironmentServiceApi.md#get_environments) | **get** /environments | Returns UUIDs of existing analyses.
*EnvironmentServiceApi* | [**get_published_environments**](docs/EnvironmentServiceApi.md#get_published_environments) | **get** /environments/published | 
*EnvironmentServiceApi* | [**get_subscribed_environments**](docs/EnvironmentServiceApi.md#get_subscribed_environments) | **get** /environments/subscriptions | 
*EnvironmentServiceApi* | [**publish_environment**](docs/EnvironmentServiceApi.md#publish_environment) | **post** /environments | 
*EnvironmentServiceApi* | [**subscribe_environment**](docs/EnvironmentServiceApi.md#subscribe_environment) | **post** /environments/{uuid}/subscribe | 
*EnvironmentServiceApi* | [**update_environment**](docs/EnvironmentServiceApi.md#update_environment) | **put** /environments/{uuid} | 
*InvoiceServiceApi* | [**get_billing_portal_url**](docs/InvoiceServiceApi.md#get_billing_portal_url) | **get** /invoices/portal | 
*InvoiceServiceApi* | [**get_invoices**](docs/InvoiceServiceApi.md#get_invoices) | **get** /invoices | 
*NodeServiceApi* | [**create_node**](docs/NodeServiceApi.md#create_node) | **post** /nodes/v2 | 
*NodeServiceApi* | [**get_bootstrap**](docs/NodeServiceApi.md#get_bootstrap) | **get** /nodes/bootstrap | 
*NodeServiceApi* | [**get_node_licenses**](docs/NodeServiceApi.md#get_node_licenses) | **get** /nodes/licenses | 
*NodeServiceApi* | [**get_node_v2**](docs/NodeServiceApi.md#get_node_v2) | **get** /nodes/v2/{ident} | 
*NodeServiceApi* | [**get_nodes2**](docs/NodeServiceApi.md#get_nodes2) | **get** /nodes | 
*NodeServiceApi* | [**get_nodes_v2**](docs/NodeServiceApi.md#get_nodes_v2) | **get** /nodes/v2 | Returns UUIDs of existing analyses.
*NodeServiceApi* | [**kill_node**](docs/NodeServiceApi.md#kill_node) | **post** /nodes/v2/{ident}/kill | 
*NodeServiceApi* | [**post_node_info**](docs/NodeServiceApi.md#post_node_info) | **post** /nodes | 
*NodeServiceApi* | [**post_node_info_0**](docs/NodeServiceApi.md#post_node_info_0) | **post** /nodes/create | 
*OrganizationServiceApi* | [**get_nodes3**](docs/OrganizationServiceApi.md#get_nodes3) | **get** /organizations | 
*RequestServiceApi* | [**add_data**](docs/RequestServiceApi.md#add_data) | **post** /requests/{uuid}/data | Is used to upload and park the data till the AnalysisRequest gets processed.
*RequestServiceApi* | [**cancel_processing**](docs/RequestServiceApi.md#cancel_processing) | **post** /requests/{uuid}/cancel | Endpoint is called from the Analysis Processing entity to submit the result.
*RequestServiceApi* | [**create_new_request**](docs/RequestServiceApi.md#create_new_request) | **post** /requests | Endpoint used to create a new Analysis Request.
*RequestServiceApi* | [**get_actions**](docs/RequestServiceApi.md#get_actions) | **get** /requests/actions | 
*RequestServiceApi* | [**get_agent**](docs/RequestServiceApi.md#get_agent) | **get** /requests/{uuid}/agent | Is called from the APE to request all parked datasets.
*RequestServiceApi* | [**get_all_requests**](docs/RequestServiceApi.md#get_all_requests) | **get** /requests | Returns UUIDs of existing analyses.
*RequestServiceApi* | [**get_data**](docs/RequestServiceApi.md#get_data) | **get** /requests/{uuid}/data | Is called from the APE to request all parked datasets.
*RequestServiceApi* | [**get_request**](docs/RequestServiceApi.md#get_request) | **get** /requests/{uuid} | Returns the details for certain Request.
*RequestServiceApi* | [**get_result**](docs/RequestServiceApi.md#get_result) | **get** /requests/{uuid}/result | Can be called from creator to request the AnalysisResult.
*RequestServiceApi* | [**get_status**](docs/RequestServiceApi.md#get_status) | **get** /requests/stats | 
*RequestServiceApi* | [**give_consent**](docs/RequestServiceApi.md#give_consent) | **post** /requests/{uuid}/consent | Used to give consent for request.
*RequestServiceApi* | [**init_request_parameters**](docs/RequestServiceApi.md#init_request_parameters) | **post** /requests/{uuid} | Endpoint used initialized addition datacollection parameters for requester.
*RequestServiceApi* | [**set_agent**](docs/RequestServiceApi.md#set_agent) | **post** /requests/{uuid}/agent | Is called from the APE to request all parked datasets.
*RequestServiceApi* | [**set_result**](docs/RequestServiceApi.md#set_result) | **post** /requests/{uuid}/result | Endpoint is called from the Analysis Processing entity to submit the result.
*SocialPlatformServiceApi* | [**get_platforms**](docs/SocialPlatformServiceApi.md#get_platforms) | **get** /platforms | Used to Handle Incoming Webhooks from Facebook.
*SocialPlatformServiceApi* | [**listen_twitter_webhook**](docs/SocialPlatformServiceApi.md#listen_twitter_webhook) | **post** /platforms/twitter | Used to Handle Incoming Webhooks from Facebook.
*SocialPlatformServiceApi* | [**register_twitter_webhook**](docs/SocialPlatformServiceApi.md#register_twitter_webhook) | **get** /platforms/twitter | Used to Handle Incoming Webhooks from Twitter.
*SocialServiceApi* | [**get_my_profile**](docs/SocialServiceApi.md#get_my_profile) | **get** /social/profiles/me | 
*SocialServiceApi* | [**get_platforms2**](docs/SocialServiceApi.md#get_platforms2) | **get** /social | Returns all Platforms / Systems that can be Connected to the MADANA Service.
*SocialServiceApi* | [**get_ranking**](docs/SocialServiceApi.md#get_ranking) | **get** /social/ranking | Returns the Ranking by PTS within the System.
*SocialServiceApi* | [**get_social_platform_feed**](docs/SocialServiceApi.md#get_social_platform_feed) | **get** /social/feed/{platform} | 
*SocialServiceApi* | [**get_user_profile**](docs/SocialServiceApi.md#get_user_profile) | **get** /social/profiles/{username} | 
*SocialServiceApi* | [**get_user_profile_0**](docs/SocialServiceApi.md#get_user_profile_0) | **get** /social/profiles/{username}/simple | 
*SubscriptionServiceApi* | [**add_free_subscription**](docs/SubscriptionServiceApi.md#add_free_subscription) | **post** /subscriptions/saas/free | 
*SubscriptionServiceApi* | [**add_pass_trial_subscription**](docs/SubscriptionServiceApi.md#add_pass_trial_subscription) | **post** /subscriptions/paas/trial | 
*SubscriptionServiceApi* | [**get_application**](docs/SubscriptionServiceApi.md#get_application) | **get** /subscriptions/active | 
*SubscriptionServiceApi* | [**get_checkout_session**](docs/SubscriptionServiceApi.md#get_checkout_session) | **get** /subscriptions/{productname}/checkout | 
*SubscriptionServiceApi* | [**get_checkout_session2**](docs/SubscriptionServiceApi.md#get_checkout_session2) | **post** /subscriptions/{productname}/{newplan} | 
*SystemServiceApi* | [**get_all_objects**](docs/SystemServiceApi.md#get_all_objects) | **get** /system/health | 
*SystemServiceApi* | [**get_application2**](docs/SystemServiceApi.md#get_application2) | **get** /system/usage | Return the current application usage.
*UserServiceApi* | [**cancel_subscription**](docs/UserServiceApi.md#cancel_subscription) | **post** /users/{username}/subscriptions/{planname}/cancel | 
*UserServiceApi* | [**create_object**](docs/UserServiceApi.md#create_object) | **post** /users | Creates a new user object.
*UserServiceApi* | [**delete_object**](docs/UserServiceApi.md#delete_object) | **delete** /users/{username} | Deletes an User based on the provided id and securitycontext.
*UserServiceApi* | [**delete_object_0**](docs/UserServiceApi.md#delete_object_0) | **delete** /users/{username}/social/{platform}/{ident} | Deletes linked account from the user and securitycontext.
*UserServiceApi* | [**get_avatars**](docs/UserServiceApi.md#get_avatars) | **get** /users/{username}/avatars | 
*UserServiceApi* | [**get_certificates**](docs/UserServiceApi.md#get_certificates) | **get** /users/{username}/certificates | 
*UserServiceApi* | [**get_enclave_history**](docs/UserServiceApi.md#get_enclave_history) | **get** /users/{username}/enclavehistory | 
*UserServiceApi* | [**get_object2**](docs/UserServiceApi.md#get_object2) | **get** /users/{username} | 
*UserServiceApi* | [**set_avatar**](docs/UserServiceApi.md#set_avatar) | **post** /users/{username}/avatars | 
*UserServiceApi* | [**set_settings**](docs/UserServiceApi.md#set_settings) | **post** /users/{username}/settings | 
*UserServiceApi* | [**update_object**](docs/UserServiceApi.md#update_object) | **put** /users/{username} | Updates Userproperties based on the provided user object.


## Documentation For Models

 - [JsonDiskConfig](docs/JsonDiskConfig.md)
 - [JsonEnclavePort](docs/JsonEnclavePort.md)
 - [JsonEnclaveProcess](docs/JsonEnclaveProcess.md)
 - [JsonEnclaveRunRequest](docs/JsonEnclaveRunRequest.md)
 - [JsonEnclaveRunningAttestation](docs/JsonEnclaveRunningAttestation.md)
 - [JsonEnclaveRunningAttestationApproval](docs/JsonEnclaveRunningAttestationApproval.md)
 - [JsonEnclaveRunningAttestationApprovalAllOf](docs/JsonEnclaveRunningAttestationApprovalAllOf.md)
 - [JsonEnvironment](docs/JsonEnvironment.md)
 - [JsonEnvironmentPublishingRequest](docs/JsonEnvironmentPublishingRequest.md)
 - [JsonIpfsSystemInfo](docs/JsonIpfsSystemInfo.md)
 - [JsonKubernetesEnclave](docs/JsonKubernetesEnclave.md)
 - [JsonKubernetesEnclaveAllOf](docs/JsonKubernetesEnclaveAllOf.md)
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
 - [JsonNodeRunRequest](docs/JsonNodeRunRequest.md)
 - [JsonProcess](docs/JsonProcess.md)
 - [JsonRunConfig](docs/JsonRunConfig.md)
 - [JsonSgxInfo](docs/JsonSgxInfo.md)
 - [JsonSignedData](docs/JsonSignedData.md)
 - [JsonV1Event](docs/JsonV1Event.md)
 - [JsonV1EventList](docs/JsonV1EventList.md)
 - [JsonV1EventSeries](docs/JsonV1EventSeries.md)
 - [JsonV1EventSource](docs/JsonV1EventSource.md)
 - [JsonV1ListMeta](docs/JsonV1ListMeta.md)
 - [JsonV1ManagedFieldsEntry](docs/JsonV1ManagedFieldsEntry.md)
 - [JsonV1ObjectMeta](docs/JsonV1ObjectMeta.md)
 - [JsonV1ObjectReference](docs/JsonV1ObjectReference.md)
 - [JsonV1OwnerReference](docs/JsonV1OwnerReference.md)
 - [JsonWireguardInterface](docs/JsonWireguardInterface.md)
 - [JsonWireguardInterfaceAllOf](docs/JsonWireguardInterfaceAllOf.md)
 - [XmlNs0DiskConfig](docs/XmlNs0DiskConfig.md)
 - [XmlNs0DiskConfigAllOf](docs/XmlNs0DiskConfigAllOf.md)
 - [XmlNs0EnclavePort](docs/XmlNs0EnclavePort.md)
 - [XmlNs0EnclavePortAllOf](docs/XmlNs0EnclavePortAllOf.md)
 - [XmlNs0EnclaveProcess](docs/XmlNs0EnclaveProcess.md)
 - [XmlNs0EnclaveProcessAllOf](docs/XmlNs0EnclaveProcessAllOf.md)
 - [XmlNs0EnclaveRunningAttestation](docs/XmlNs0EnclaveRunningAttestation.md)
 - [XmlNs0EnclaveRunningAttestationAllOf](docs/XmlNs0EnclaveRunningAttestationAllOf.md)
 - [XmlNs0EnclaveRunningAttestationApproval](docs/XmlNs0EnclaveRunningAttestationApproval.md)
 - [XmlNs0EnclaveRunningAttestationApprovalAllOf](docs/XmlNs0EnclaveRunningAttestationApprovalAllOf.md)
 - [XmlNs0Environment](docs/XmlNs0Environment.md)
 - [XmlNs0EnvironmentAllOf](docs/XmlNs0EnvironmentAllOf.md)
 - [XmlNs0InputStream](docs/XmlNs0InputStream.md)
 - [XmlNs0IpfsSystemInfo](docs/XmlNs0IpfsSystemInfo.md)
 - [XmlNs0IpfsSystemInfoAllOf](docs/XmlNs0IpfsSystemInfoAllOf.md)
 - [XmlNs0KubernetesEnclave](docs/XmlNs0KubernetesEnclave.md)
 - [XmlNs0KubernetesEnclaveAllOf](docs/XmlNs0KubernetesEnclaveAllOf.md)
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
 - [XmlNs0SgxInfo](docs/XmlNs0SgxInfo.md)
 - [XmlNs0SgxInfoAllOf](docs/XmlNs0SgxInfoAllOf.md)
 - [XmlNs0SignedData](docs/XmlNs0SignedData.md)
 - [XmlNs0SignedDataAllOf](docs/XmlNs0SignedDataAllOf.md)
 - [XmlNs0WireguardInterface](docs/XmlNs0WireguardInterface.md)
 - [XmlNs0WireguardInterfaceAllOf](docs/XmlNs0WireguardInterfaceAllOf.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



