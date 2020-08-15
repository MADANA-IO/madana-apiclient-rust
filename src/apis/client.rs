use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    account_service_api: Box<dyn crate::apis::AccountServiceApi>,
    authentication_service_api: Box<dyn crate::apis::AuthenticationServiceApi>,
    certificate_service_api: Box<dyn crate::apis::CertificateServiceApi>,
    data_collection_service_api: Box<dyn crate::apis::DataCollectionServiceApi>,
    node_service_api: Box<dyn crate::apis::NodeServiceApi>,
    organization_service_api: Box<dyn crate::apis::OrganizationServiceApi>,
    request_service_api: Box<dyn crate::apis::RequestServiceApi>,
    social_platform_service_api: Box<dyn crate::apis::SocialPlatformServiceApi>,
    social_service_api: Box<dyn crate::apis::SocialServiceApi>,
    system_service_api: Box<dyn crate::apis::SystemServiceApi>,
    user_service_api: Box<dyn crate::apis::UserServiceApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            account_service_api: Box::new(crate::apis::AccountServiceApiClient::new(rc.clone())),
            authentication_service_api: Box::new(crate::apis::AuthenticationServiceApiClient::new(rc.clone())),
            certificate_service_api: Box::new(crate::apis::CertificateServiceApiClient::new(rc.clone())),
            data_collection_service_api: Box::new(crate::apis::DataCollectionServiceApiClient::new(rc.clone())),
            node_service_api: Box::new(crate::apis::NodeServiceApiClient::new(rc.clone())),
            organization_service_api: Box::new(crate::apis::OrganizationServiceApiClient::new(rc.clone())),
            request_service_api: Box::new(crate::apis::RequestServiceApiClient::new(rc.clone())),
            social_platform_service_api: Box::new(crate::apis::SocialPlatformServiceApiClient::new(rc.clone())),
            social_service_api: Box::new(crate::apis::SocialServiceApiClient::new(rc.clone())),
            system_service_api: Box::new(crate::apis::SystemServiceApiClient::new(rc.clone())),
            user_service_api: Box::new(crate::apis::UserServiceApiClient::new(rc.clone())),
        }
    }

    pub fn account_service_api(&self) -> &dyn crate::apis::AccountServiceApi{
        self.account_service_api.as_ref()
    }

    pub fn authentication_service_api(&self) -> &dyn crate::apis::AuthenticationServiceApi{
        self.authentication_service_api.as_ref()
    }

    pub fn certificate_service_api(&self) -> &dyn crate::apis::CertificateServiceApi{
        self.certificate_service_api.as_ref()
    }

    pub fn data_collection_service_api(&self) -> &dyn crate::apis::DataCollectionServiceApi{
        self.data_collection_service_api.as_ref()
    }

    pub fn node_service_api(&self) -> &dyn crate::apis::NodeServiceApi{
        self.node_service_api.as_ref()
    }

    pub fn organization_service_api(&self) -> &dyn crate::apis::OrganizationServiceApi{
        self.organization_service_api.as_ref()
    }

    pub fn request_service_api(&self) -> &dyn crate::apis::RequestServiceApi{
        self.request_service_api.as_ref()
    }

    pub fn social_platform_service_api(&self) -> &dyn crate::apis::SocialPlatformServiceApi{
        self.social_platform_service_api.as_ref()
    }

    pub fn social_service_api(&self) -> &dyn crate::apis::SocialServiceApi{
        self.social_service_api.as_ref()
    }

    pub fn system_service_api(&self) -> &dyn crate::apis::SystemServiceApi{
        self.system_service_api.as_ref()
    }

    pub fn user_service_api(&self) -> &dyn crate::apis::UserServiceApi{
        self.user_service_api.as_ref()
    }

}
