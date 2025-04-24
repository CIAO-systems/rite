use crate::ciao::core::config::{
    configuration_service_client::ConfigurationServiceClient, ConfigurationValuesRequest,
    ConfigurationValuesResponse,
};
use grpc_utils_rs::grpc::{channel, interceptor::{CompositeInterceptor, Interceptors}};
use tonic::{service::interceptor::InterceptedService, transport::Channel};

#[derive(Debug)]
pub struct ConfigurationClient {
    inner: ConfigurationServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl ConfigurationClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: ConfigurationServiceClient::with_interceptor(
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
    ) -> &ConfigurationServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut ConfigurationServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }

    pub async fn get_values(
        &mut self,
        key: Option<&str>,
    ) -> Result<ConfigurationValuesResponse, Box<dyn std::error::Error>> {
        let keys = match key {
            Some(key) => vec![String::from(key)],
            None => vec![],
        };
        let request = ConfigurationValuesRequest { keys };
        let response = self.inner.get_configuration_values(request).await?;
        Ok(response.into_inner())
    }
}
