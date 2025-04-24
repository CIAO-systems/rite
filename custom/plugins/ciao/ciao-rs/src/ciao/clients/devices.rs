use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};
use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::ciao::devices::{
    device_service_client::DeviceServiceClient, DeviceConfigurationRequest,
    DeviceConfigurationResponse,
};

#[derive(Debug)]
pub struct DeviceClient {
    inner: DeviceServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl DeviceClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: DeviceServiceClient::with_interceptor(
                        channel,
                        CompositeInterceptor::new(interceptors),
                    ),
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Returns the encapsulated service client
    pub fn inner(&self) -> &DeviceServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut DeviceServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }

    pub async fn get_device_configuration(
        &mut self,
        external_id: &str,
    ) -> Result<DeviceConfigurationResponse, Box<dyn std::error::Error>> {
        let request = DeviceConfigurationRequest {
            external_id: String::from(external_id),
        };
        let response = self.inner.get_device_configuration(request).await?;
        Ok(response.into_inner())
    }
}
