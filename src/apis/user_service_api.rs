/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.14-master.20
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct UserServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> UserServiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UserServiceApiClient<C> {
        UserServiceApiClient {
            configuration,
        }
    }
}

pub trait UserServiceApi {
    fn create_object(&self, referrer: Option<&str>, body: Option<crate::models::JsonMdnUser>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn delete_object(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn delete_object_0(&self, ident: &str, platform: &str, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_avatars(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_certificates(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_object2(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn set_avatar(&self, username: &str, body: Option<crate::models::JsonMdnUserProfileImage>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn set_settings(&self, username: &str, body: Option<crate::models::JsonMdnUserSetting>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn update_object(&self, username: &str, body: Option<crate::models::JsonMdnUser>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>UserServiceApi for UserServiceApiClient<C> {
    fn create_object(&self, referrer: Option<&str>, body: Option<crate::models::JsonMdnUser>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/users".to_string())
        ;
        if let Some(ref s) = referrer {
            req = req.with_query_param("referrer".to_string(), s.to_string());
        }
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn delete_object(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/users/{username}".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_object_0(&self, ident: &str, platform: &str, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/users/{username}/social/{platform}/{ident}".to_string())
        ;
        req = req.with_path_param("ident".to_string(), ident.to_string());
        req = req.with_path_param("platform".to_string(), platform.to_string());
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_avatars(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{username}/avatars".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_certificates(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{username}/certificates".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_object2(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{username}".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

    fn set_avatar(&self, username: &str, body: Option<crate::models::JsonMdnUserProfileImage>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/users/{username}/avatars".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn set_settings(&self, username: &str, body: Option<crate::models::JsonMdnUserSetting>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/users/{username}/settings".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn update_object(&self, username: &str, body: Option<crate::models::JsonMdnUser>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/users/{username}".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

}
