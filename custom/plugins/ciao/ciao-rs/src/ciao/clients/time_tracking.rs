use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::ciao::{
    common::TimeRange,
    time_tracking::{
        time_tracking_service_client::TimeTrackingServiceClient, GetRequest, GetResponse,
        ListRequest, ListResponse,
    },
};
use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};

pub mod cost_centers;
pub mod project_tasks;
pub mod projects;
pub mod time_type;

#[derive(Debug)]
pub struct TimeTrackingClient {
    inner: TimeTrackingServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl TimeTrackingClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: TimeTrackingServiceClient::with_interceptor(
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
    ) -> &TimeTrackingServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut TimeTrackingServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }

    pub async fn get(&mut self, id: &str) -> Result<GetResponse, Box<dyn std::error::Error>> {
        let request = GetRequest {
            id: String::from(id),
        };
        let response = self.inner.get(request).await?;
        Ok(response.into_inner())
    }

    pub async fn list(
        &mut self,
        time_range: Option<TimeRange>,
        user_id: Option<String>,
        creator_id: Option<String>,
        time_type_id: Option<String>,
    ) -> Result<
        impl futures::Stream<Item = Result<ListResponse, tonic::Status>>,
        Box<dyn std::error::Error>,
    > {
        let request = ListRequest {
            time_range,
            user_id,
            creator_id,
            time_type_id,
        };
        let response = self.inner.list(request).await?;
        Ok(response.into_inner())
    }
}
