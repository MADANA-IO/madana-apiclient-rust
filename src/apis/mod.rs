use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod account_service_api;
pub use self::account_service_api::{ AccountServiceApi, AccountServiceApiClient };
mod authentication_service_api;
pub use self::authentication_service_api::{ AuthenticationServiceApi, AuthenticationServiceApiClient };
mod certificate_service_api;
pub use self::certificate_service_api::{ CertificateServiceApi, CertificateServiceApiClient };
mod data_collection_service_api;
pub use self::data_collection_service_api::{ DataCollectionServiceApi, DataCollectionServiceApiClient };
mod enclave_service_api;
pub use self::enclave_service_api::{ EnclaveServiceApi, EnclaveServiceApiClient };
mod environment_service_api;
pub use self::environment_service_api::{ EnvironmentServiceApi, EnvironmentServiceApiClient };
mod node_service_api;
pub use self::node_service_api::{ NodeServiceApi, NodeServiceApiClient };
mod organization_service_api;
pub use self::organization_service_api::{ OrganizationServiceApi, OrganizationServiceApiClient };
mod request_service_api;
pub use self::request_service_api::{ RequestServiceApi, RequestServiceApiClient };
mod social_platform_service_api;
pub use self::social_platform_service_api::{ SocialPlatformServiceApi, SocialPlatformServiceApiClient };
mod social_service_api;
pub use self::social_service_api::{ SocialServiceApi, SocialServiceApiClient };
mod system_service_api;
pub use self::system_service_api::{ SystemServiceApi, SystemServiceApiClient };
mod user_service_api;
pub use self::user_service_api::{ UserServiceApi, UserServiceApiClient };

pub mod configuration;
pub mod client;
