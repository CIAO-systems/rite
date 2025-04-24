use crate::ciao::accounts::account_service_client::AccountServiceClient;
use crate::ciao::accounts::get_request::Identity;
use crate::ciao::accounts::GetRequest;
use crate::ciao::accounts::GetResponse;
use crate::ciao::accounts::ListRequest;
use crate::ciao::accounts::ListResponse;
use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};
use tonic::{service::interceptor::InterceptedService, transport::Channel};

#[derive(Debug)]
pub struct AccountClient {
    inner: AccountServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl AccountClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: AccountServiceClient::with_interceptor(
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
    ) -> &AccountServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut AccountServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }

    pub async fn get(&mut self, id: &str) -> Result<GetResponse, Box<dyn std::error::Error>> {
        let request = GetRequest {
            identity: Some(Identity::Id(String::from(id))),
        };
        let response = self.inner.get(request).await?;
        Ok(response.into_inner())
    }

    pub async fn list(
        &mut self,
    ) -> Result<
        impl futures::Stream<Item = Result<ListResponse, tonic::Status>>,
        Box<dyn std::error::Error>,
    > {
        let request = ListRequest {};
        let response = self.inner.list(request).await?;
        Ok(response.into_inner())
    }
}
