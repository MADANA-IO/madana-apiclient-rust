/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.56
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `add_data`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddDataError {
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `cancel_processing`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelProcessingError {
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_new_request`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewRequestError {
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_actions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_agent`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentError {
    Status403(),
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_all_requests`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllRequestsError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_data`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDataError {
    Status403(),
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_request`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRequestError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_result`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetResultError {
    Status403(),
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatusError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `give_consent`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GiveConsentError {
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `init_request_parameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitRequestParametersError {
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_agent`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetAgentError {
    Status403(),
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_result`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetResultError {
    Status404(),
    Status406(),
    Status500(),
    UnknownValue(serde_json::Value),
}


/// Is used to upload and park the data till the AnalysisRequest gets processed
pub async fn add_data(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>, body: Option<crate::models::JsonSignedData>) -> Result<std::path::PathBuf, Error<AddDataError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/data", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Endpoint is called from the Analysis Processing entity to submit the result
pub async fn cancel_processing(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>, body: Option<crate::models::JsonSignedData>) -> Result<std::path::PathBuf, Error<CancelProcessingError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/cancel", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CancelProcessingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Endpoint used to create a new Analysis Request
pub async fn create_new_request(configuration: &configuration::Configuration, authorization: Option<&str>, body: Option<crate::models::JsonSignedData>) -> Result<String, Error<CreateNewRequestError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateNewRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_actions(configuration: &configuration::Configuration, limit: Option<&str>, offset: Option<&str>) -> Result<std::path::PathBuf, Error<GetActionsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/actions", configuration.base_path);
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
        let local_var_entity: Option<GetActionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Is called from the APE to request all parked datasets. Returns the transmitted data for certain Request. When requesting the data, the status of the request is automatically set to processing.
pub async fn get_agent(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>) -> Result<std::path::PathBuf, Error<GetAgentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/agent", configuration.base_path, uuid=crate::apis::urlencode(uuid));
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
        let local_var_entity: Option<GetAgentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns UUIDs of existing analyses.
pub async fn get_all_requests(configuration: &configuration::Configuration, authorization: Option<&str>, created: Option<&str>, history: Option<&str>, limit: Option<&str>, new: Option<&str>, offset: Option<&str>, preview: Option<&str>, ready: Option<&str>) -> Result<std::path::PathBuf, Error<GetAllRequestsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = created {
        local_var_req_builder = local_var_req_builder.query(&[("created", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = history {
        local_var_req_builder = local_var_req_builder.query(&[("history", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = new {
        local_var_req_builder = local_var_req_builder.query(&[("new", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = preview {
        local_var_req_builder = local_var_req_builder.query(&[("preview", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ready {
        local_var_req_builder = local_var_req_builder.query(&[("ready", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllRequestsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Is called from the APE to request all parked datasets. Returns the transmitted data for certain Request. When requesting the data, the status of the request is automatically set to processing.
pub async fn get_data(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>) -> Result<crate::models::JsonSignedData, Error<GetDataError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/data", configuration.base_path, uuid=crate::apis::urlencode(uuid));
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
        let local_var_entity: Option<GetDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the details for certain Request. When requesting an analysis a view of the analysis is stored in the database
pub async fn get_request(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>) -> Result<std::path::PathBuf, Error<GetRequestError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}", configuration.base_path, uuid=crate::apis::urlencode(uuid));
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
        let local_var_entity: Option<GetRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Can be called from creator to request the AnalysisResult.
pub async fn get_result(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>) -> Result<std::path::PathBuf, Error<GetResultError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/result", configuration.base_path, uuid=crate::apis::urlencode(uuid));
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
        let local_var_entity: Option<GetResultError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_status(configuration: &configuration::Configuration, ) -> Result<std::path::PathBuf, Error<GetStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/stats", configuration.base_path);
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
        let local_var_entity: Option<GetStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Used to give consent for request. If the Endpoint is called from the creator of the Analysis, the status of the request is set to completed. If called by another is interpreted as giving consent to participate.
pub async fn give_consent(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>) -> Result<std::path::PathBuf, Error<GiveConsentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/consent", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

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
        let local_var_entity: Option<GiveConsentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Endpoint used initialized addition datacollection parameters for requester
pub async fn init_request_parameters(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>, body: Option<&str>) -> Result<String, Error<InitRequestParametersError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<InitRequestParametersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Is called from the APE to request all parked datasets. Returns the transmitted data for certain Request. When requesting the data, the status of the request is automatically set to processing.
pub async fn set_agent(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>) -> Result<std::path::PathBuf, Error<SetAgentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/agent", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

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
        let local_var_entity: Option<SetAgentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Endpoint is called from the Analysis Processing entity to submit the result
pub async fn set_result(configuration: &configuration::Configuration, uuid: &str, authorization: Option<&str>, body: Option<crate::models::JsonSignedData>) -> Result<std::path::PathBuf, Error<SetResultError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/requests/{uuid}/result", configuration.base_path, uuid=crate::apis::urlencode(uuid));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = authorization {
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetResultError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

