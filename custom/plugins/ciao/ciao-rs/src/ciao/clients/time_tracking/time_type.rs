use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::ciao::time_tracking::time_type::{
    time_type_service_client::TimeTypeServiceClient, GetRequest, GetResponse, ListRequest,
    ListResponse,
};
use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};

#[derive(Debug)]
pub struct TimeTypeClient {
    inner: TimeTypeServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl TimeTypeClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: TimeTypeServiceClient::with_interceptor(
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
    ) -> &TimeTypeServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut TimeTypeServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
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
        absence: Option<bool>,
        bookable: Option<bool>,
    ) -> Result<
        impl futures::Stream<Item = Result<ListResponse, tonic::Status>>,
        Box<dyn std::error::Error>,
    > {
        let request = ListRequest { absence, bookable };
        let response = self.inner.list(request).await?;
        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use crate::ciao::{
        time_tracking::time_type::{
            CreateRequest, GetRequest, ListRequest, TimeType, TimeTypeOptions, UpdateRequest,
        },
        ClientManager,
    };

    use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};
    use tokio_stream::StreamExt;

    #[tokio::test]
    #[ignore = "for manual testing"]
    async fn test_list() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = ClientManager::new(
            "http://localhost:50051",
            interceptors!(APIKeyClientInterceptor::new(
                "top-secret-api-key".to_string()
            )),
        )
        .await?;
        let client = manager.time_type_client.inner_mut();
        let request = ListRequest {
            absence: None,
            bookable: None,
        };

        let mut stream = client.list(request).await?.into_inner();
        let mut count = 0;
        while let Some(element) = stream.next().await {
            match element {
                Ok(response) => {
                    for list_time_type in response.time_types {
                        count = count + 1;

                        // Retrieve the time type by id and assert their
                        // fields match
                        let tt2 = client
                            .get(GetRequest {
                                id: list_time_type.id.clone(),
                            })
                            .await?
                            .into_inner();
                        if let Some(gotten_time_type) = tt2.time_type {
                            assert_eq!(gotten_time_type.name, list_time_type.name);
                            assert_eq!(gotten_time_type.color, list_time_type.color);
                            assert_eq!(gotten_time_type.icon, list_time_type.icon);
                            assert_eq!(gotten_time_type.options, list_time_type.options);
                            assert_eq!(gotten_time_type.shorthand, list_time_type.shorthand);
                        } else {
                            panic!("Unable to get time_type: {:?}", list_time_type);
                        }
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(e.into());
                }
            }
        }

        assert_eq!(count, 15);

        Ok(())
    }

    #[tokio::test]
    #[ignore = "for manual testing"]
    async fn test_create_and_update() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = ClientManager::new(
            "http://localhost:50051",
            interceptors!(APIKeyClientInterceptor::new(
                "top-secret-api-key".to_string()
            )),
        )
        .await?;
        let client = manager.time_type_client.inner_mut();
        let time_type = TimeType {
            id: String::from("fancy-id"),
            name: String::from("fancy-name"),
            shorthand: String::from("FD"),
            color: None,
            icon: None,
            options: Some(TimeTypeOptions {
                bookable: false,
                absence: true,
            }),
        };

        let response = client
            .create(CreateRequest {
                time_type: Some(time_type.clone()),
            })
            .await?
            .into_inner();

        assert_eq!(response.time_type, Some(time_type.clone()));

        let response = client
            .get(GetRequest {
                id: time_type.id.clone(),
            })
            .await?
            .into_inner();
        assert_eq!(response.time_type, Some(time_type.clone()));

        let mut time_type = time_type.clone();
        time_type.name = String::from("a completely different name");
        client
            .update(UpdateRequest {
                time_type: Some(time_type.clone()),
            })
            .await?
            .into_inner();

        let response = client
            .get(GetRequest {
                id: time_type.id.clone(),
            })
            .await?
            .into_inner();
        assert_eq!(response.time_type, Some(time_type.clone()));

        Ok(())
    }
}
