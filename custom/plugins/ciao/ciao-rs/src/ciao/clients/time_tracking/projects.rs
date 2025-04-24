use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::ciao::time_tracking::project::project_service_client::ProjectServiceClient;
use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};

#[derive(Debug)]
pub struct ProjectClient {
    inner: ProjectServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl ProjectClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: ProjectServiceClient::with_interceptor(
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
    ) -> &ProjectServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut ProjectServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};
    use tonic::Status;
    use uuid::Uuid;

    use super::ProjectClient;
    use crate::ciao::time_tracking::project::{CreateRequest, DeleteRequest, GetRequest, Project};
    use std::error::Error;

    async fn create_client() -> Result<ProjectClient, Box<dyn Error>> {
        let pc = ProjectClient::new(
            "http://localhost:50051",
            interceptors!(APIKeyClientInterceptor::new(
                "top-secret-api-key".to_string()
            )),
        )
        .await?;
        Ok(pc)
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_project_client_get() -> Result<(), Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new()?;
        let _: Result<(), Box<dyn Error>> = rt.block_on(async {
            let mut pc = create_client().await?;

            match pc
                .inner_mut()
                .get(GetRequest {
                    id: "2fae3e46-f4d2-4300-9e36-5159f9de9c9f".to_string(),
                })
                .await
            {
                Ok(r) => {
                    let p = r.into_inner();
                    println!("{:?}", p);
                    assert_eq!(
                        "2fae3e46-f4d2-4300-9e36-5159f9de9c9f",
                        p.project.unwrap().id
                    );
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                }
            }

            Ok(())
        });

        Ok(())
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_project_client_create_and_delete() -> Result<(), Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new()?;
        let _: Result<(), Box<dyn Error>> = rt.block_on(async {
            let mut pc = create_client().await?;

            let new_project = Project {
                id: Uuid::new_v4().to_string(),
                external_id: Some(Uuid::new_v4().to_string()),
                name: Uuid::new_v4().to_string(),
                start_date: None,
                end_date: None,
                closed_date: None,
                parent_id: None,
            };

            // Create the new project
            let create_response = pc
                .inner_mut()
                .create(CreateRequest {
                    project: Some(new_project.clone()),
                })
                .await?;

            let response = create_response.into_inner();
            println!("{:?}", response);
            assert_eq!(response.project.unwrap().id, new_project.id);

            // Delete it again
            let delete_response = pc
                .inner_mut()
                .delete(DeleteRequest {
                    id: new_project.id.clone(),
                })
                .await?;

            let response = delete_response.into_inner();
            println!("{:?}", response);
            assert_eq!(response.message, "");

            // Check, it is no longer there
            let result = pc
                .inner_mut()
                .get(GetRequest {
                    id: new_project.id.clone(),
                })
                .await;

            assert!(result.is_err());
            let error = result.err().unwrap();
            let expected = Status::not_found("Project not found");
            assert_eq!(error.code(), expected.code());
            assert_eq!(error.message(), expected.message());

            Ok(())
        });

        Ok(())
    }
}
