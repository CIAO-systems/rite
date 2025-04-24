use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::ciao::time_tracking::cost_center::cost_center_service_client::CostCenterServiceClient;
use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};

#[derive(Debug)]
pub struct CostCenterClient {
    inner: CostCenterServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl CostCenterClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: CostCenterServiceClient::with_interceptor(
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
    ) -> &CostCenterServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut CostCenterServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};

    use super::CostCenterClient;
    use crate::ciao::time_tracking::cost_center::GetRequest;
    use std::error::Error;

    #[test]
    #[ignore = "for manual testing"]
    fn test_cost_center_client() -> Result<(), Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new()?;
        let _: Result<(), Box<dyn Error>> = rt.block_on(async {
            let mut pc = CostCenterClient::new(
                "http://localhost:50051",
                interceptors!(APIKeyClientInterceptor::new(
                    "to-secret-api-key".to_string()
                )),
            )
            .await?;

            match pc
                .inner_mut()
                .get(GetRequest {
                    id: "cc-001".to_string(),
                })
                .await
            {
                Ok(r) => {
                    let response = r.into_inner();
                    println!("{:?}", response);
                    assert_eq!("HR Department", response.cost_center.unwrap().name);
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                }
            }

            Ok(())
        });

        Ok(())
    }
}
