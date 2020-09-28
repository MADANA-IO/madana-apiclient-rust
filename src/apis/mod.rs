use reqwest;
use serde_json;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod account_service_api;
pub mod authentication_service_api;
pub mod certificate_service_api;
pub mod data_collection_service_api;
pub mod enclave_service_api;
pub mod environment_service_api;
pub mod node_service_api;
pub mod organization_service_api;
pub mod request_service_api;
pub mod social_platform_service_api;
pub mod social_service_api;
pub mod system_service_api;
pub mod user_service_api;

pub mod configuration;
