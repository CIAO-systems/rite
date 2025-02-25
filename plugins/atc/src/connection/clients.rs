use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};
use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::com::atoss::atc::protobuf::data_set_service_client::DataSetServiceClient;

pub mod manager;

#[derive(Debug)]
pub struct DataSetClient {
    inner: DataSetServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl DataSetClient {
    async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: DataSetServiceClient::with_interceptor(
                        channel,
                        CompositeInterceptor::new(interceptors),
                    ),
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut DataSetServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }
}
