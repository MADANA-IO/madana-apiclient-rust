/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.14
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `authenticate_application`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticateApplicationError {
    Status401(),
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `authenticate_ethereum_wallet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticateEthereumWalletError {
    Status417(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `authenticate_user`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticateUserError {
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `authenticate_with_ethereum_challenge`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticateWithEthereumChallengeError {
    Status417(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_fractal_authentication_url`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFractalAuthenticationUrlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_nonce_for_ethereum_wallet`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNonceForEthereumWalletError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_object`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_twitter_authentication_url`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTwitterAuthenticationUrlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_facebook_uid`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetFacebookUidError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_fractal_uid`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetFractalUidError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_twitter_uid`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetTwitterUidError {
    UnknownValue(serde_json::Value),
}


/// Authenticates a new application and returns the token
pub async fn authenticate_application(configuration: &configuration::Configuration, body: Option<crate::models::JsonMdnCertificate>) -> Result<crate::models::JsonMdnToken, Error<AuthenticateApplicationError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/application", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthenticateApplicationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn authenticate_ethereum_wallet(configuration: &configuration::Configuration, wallet: &str, body: Option<crate::models::JsonMdnOAuthToken>) -> Result<std::path::PathBuf, Error<AuthenticateEthereumWalletError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/ethereum/{wallet}", configuration.base_path, wallet=crate::apis::urlencode(wallet));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthenticateEthereumWalletError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Authenticates a new user and returns the token (  forbidden if the credentials cannot be validated )
pub async fn authenticate_user(configuration: &configuration::Configuration, body: Option<crate::models::JsonMdnUserCredentials>) -> Result<crate::models::JsonMdnToken, Error<AuthenticateUserError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthenticateUserError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn authenticate_with_ethereum_challenge(configuration: &configuration::Configuration, wallet: &str, body: Option<crate::models::JsonMdnOAuthToken>) -> Result<std::path::PathBuf, Error<AuthenticateWithEthereumChallengeError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/ethereum/{wallet}/challenge", configuration.base_path, wallet=crate::apis::urlencode(wallet));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthenticateWithEthereumChallengeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the AUthorization URL to verify a Twitter Accounts
pub async fn get_fractal_authentication_url(configuration: &configuration::Configuration, ) -> Result<std::path::PathBuf, Error<GetFractalAuthenticationUrlError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/fractal", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFractalAuthenticationUrlError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a nonce for the client which is used as content for the to be created signature
pub async fn get_nonce_for_ethereum_wallet(configuration: &configuration::Configuration, wallet: &str, authorization: Option<&str>) -> Result<crate::models::JsonMdnToken, Error<GetNonceForEthereumWalletError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/ethereum/{wallet}", configuration.base_path, wallet=crate::apis::urlencode(wallet));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetNonceForEthereumWalletError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Used to validate the active connection with the API
pub async fn get_object(configuration: &configuration::Configuration, ) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<GetObjectError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetObjectError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the AUthorization URL to verify a Twitter Accounts
pub async fn get_twitter_authentication_url(configuration: &configuration::Configuration, ) -> Result<std::path::PathBuf, Error<GetTwitterAuthenticationUrlError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/twitter", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTwitterAuthenticationUrlError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Used as Callback URL when users have successfully authorized their facbeook account
pub async fn set_facebook_uid(configuration: &configuration::Configuration, body: Option<&str>) -> Result<std::path::PathBuf, Error<SetFacebookUidError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/facebook", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetFacebookUidError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn set_fractal_uid(configuration: &configuration::Configuration, body: Option<&str>) -> Result<std::path::PathBuf, Error<SetFractalUidError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/fractal", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetFractalUidError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn set_twitter_uid(configuration: &configuration::Configuration, body: Option<crate::models::JsonMdnOAuthToken>) -> Result<std::path::PathBuf, Error<SetTwitterUidError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/authentication/twitter", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetTwitterUidError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

