/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.17
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `delete_environment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEnvironmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_environment_subscription`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEnvironmentSubscriptionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_environment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEnvironmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_environments`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEnvironmentsError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_published_environments`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublishedEnvironmentsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_subscribed_environments`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSubscribedEnvironmentsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `publish_environment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishEnvironmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `subscribe_environment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubscribeEnvironmentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_environment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEnvironmentError {
    UnknownValue(serde_json::Value),
}


pub async fn delete_environment(configuration: &configuration::Configuration, uuid: &str) -> Result<std::path::PathBuf, Error<DeleteEnvironmentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/{uuid}", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_environment_subscription(configuration: &configuration::Configuration, uuid: &str) -> Result<std::path::PathBuf, Error<DeleteEnvironmentSubscriptionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/{uuid}/subscribe", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteEnvironmentSubscriptionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_environment(configuration: &configuration::Configuration, uuid: &str) -> Result<std::path::PathBuf, Error<GetEnvironmentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/{uuid}", configuration.base_path, uuid=crate::apis::urlencode(uuid));
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
        let local_var_entity: Option<GetEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns UUIDs of existing analyses.
pub async fn get_environments(configuration: &configuration::Configuration, authorization: Option<&str>, created: Option<&str>, limit: Option<&str>, name: Option<&str>, offset: Option<&str>) -> Result<std::path::PathBuf, Error<GetEnvironmentsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = created {
        local_var_req_builder = local_var_req_builder.query(&[("created", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetEnvironmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_published_environments(configuration: &configuration::Configuration, limit: Option<&str>, name: Option<&str>, offset: Option<&str>) -> Result<std::path::PathBuf, Error<GetPublishedEnvironmentsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/published", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetPublishedEnvironmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_subscribed_environments(configuration: &configuration::Configuration, limit: Option<&str>, offset: Option<&str>) -> Result<std::path::PathBuf, Error<GetSubscribedEnvironmentsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/subscriptions", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetSubscribedEnvironmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn publish_environment(configuration: &configuration::Configuration, body: Option<crate::models::JsonEnvironmentPublishingRequest>) -> Result<std::path::PathBuf, Error<PublishEnvironmentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments", configuration.base_path);
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
        let local_var_entity: Option<PublishEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn subscribe_environment(configuration: &configuration::Configuration, uuid: &str) -> Result<std::path::PathBuf, Error<SubscribeEnvironmentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/{uuid}/subscribe", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

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
        let local_var_entity: Option<SubscribeEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_environment(configuration: &configuration::Configuration, uuid: &str, body: Option<crate::models::JsonEnvironment>) -> Result<std::path::PathBuf, Error<UpdateEnvironmentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/environments/{uuid}", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

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
        let local_var_entity: Option<UpdateEnvironmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

