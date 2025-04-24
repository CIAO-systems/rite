use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};

use crate::ciao::core::auth::{
    authentication_service_client::AuthenticationServiceClient, login_request::Identity,
    LoginRequest, LoginResult,
};
use tonic::{service::interceptor::InterceptedService, transport::Channel};

#[derive(Debug)]
pub struct AuthenticationClient {
    inner: AuthenticationServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl AuthenticationClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: AuthenticationServiceClient::with_interceptor(
                        channel,
                        CompositeInterceptor::new(interceptors),
                    ),
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Returns the encapsulated service client
    pub fn inner(
        &self,
    ) -> &AuthenticationServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut AuthenticationServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }

    pub async fn login_user_id(
        &mut self,
        user_id: &str,
        secret: &str,
    ) -> Result<LoginResult, Box<dyn std::error::Error>> {
        let request = LoginRequest {
            secret: String::from(secret),
            identity: Some(Identity::UserId(String::from(user_id))),
        };
        let response = self.inner.login(request).await?;
        Ok(response.into_inner())
    }

    pub async fn login_email(
        &mut self,
        email: &str,
        secret: &str,
    ) -> Result<LoginResult, Box<dyn std::error::Error>> {
        let request = LoginRequest {
            secret: String::from(secret),
            identity: Some(Identity::Email(String::from(email))),
        };
        let response = self.inner.login(request).await?;
        Ok(response.into_inner())
    }
}
