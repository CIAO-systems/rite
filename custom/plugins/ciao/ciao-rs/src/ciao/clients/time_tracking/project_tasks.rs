use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::ciao::time_tracking::project::task::project_task_service_client::ProjectTaskServiceClient;
use grpc_utils_rs::grpc::{
    channel,
    interceptor::{CompositeInterceptor, Interceptors},
};

#[derive(Debug)]
pub struct ProjectTaskClient {
    inner: ProjectTaskServiceClient<InterceptedService<Channel, CompositeInterceptor>>,
}

impl ProjectTaskClient {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
        match Channel::from_shared(String::from(url)) {
            Ok(endpoint) => {
                let channel = channel(tls, endpoint).await?;
                Ok(Self {
                    inner: ProjectTaskServiceClient::with_interceptor(
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
    ) -> &ProjectTaskServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &self.inner
    }

    /// Returns the encapsulated service client as mutable
    pub fn inner_mut(
        &mut self,
    ) -> &mut ProjectTaskServiceClient<InterceptedService<Channel, CompositeInterceptor>> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};
    use tonic::Status;
    use uuid::Uuid;

    use super::ProjectTaskClient;
    use crate::ciao::time_tracking::project::task::{
        CreateRequest, DeleteRequest, GetRequest, ProjectTask,
    };
    use std::error::Error;

    async fn create_client() -> Result<ProjectTaskClient, Box<dyn Error>> {
        let pc = ProjectTaskClient::new(
            "http://localhost:50051",
            interceptors!(APIKeyClientInterceptor::new(
                "top-secret-api-key".to_string()
            )),
        )
        .await?;
        Ok(pc)
    }

    const TASK_ID: &str = "2b3c4d5e-6f7g-8h9i-0j1k-l2m3n4o5p6q7";

    #[test]
    #[ignore = "for manual testing"]
    fn test_project_task_client_get() -> Result<(), Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new()?;
        let _: Result<(), Box<dyn Error>> = rt.block_on(async {
            let mut pc = create_client().await?;

            match pc
                .inner_mut()
                .get(GetRequest {
                    id: TASK_ID.to_string(),
                })
                .await
            {
                Ok(r) => {
                    let task = r.into_inner();
                    println!("{:?}", task);
                    assert_eq!(TASK_ID, task.task.unwrap().id);
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }

            Ok(())
        });

        Ok(())
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_project_task_client_create_and_delete() -> Result<(), Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new()?;
        let _: Result<(), Box<dyn Error>> = rt.block_on(async {
            let mut pc = create_client().await?;

            let new_project_task = ProjectTask {
                id: Uuid::new_v4().to_string(),
                project_id: Uuid::new_v4().to_string(),
                name: Uuid::new_v4().to_string(),
            };

            // Create the new project task
            let create_response = pc
                .inner_mut()
                .create(CreateRequest {
                    task: Some(new_project_task.clone()),
                })
                .await?;

            let response = create_response.into_inner();
            println!("{:?}", response);
            assert_eq!(response.task.unwrap().id, new_project_task.clone().id);

            // Delete it again
            let delete_response = pc
                .inner_mut()
                .delete(DeleteRequest {
                    id: new_project_task.clone().id.clone(),
                })
                .await?;

            let response = delete_response.into_inner();
            println!("{:?}", response);
            assert_eq!(response.message, "");

            // Check, it is no longer there
            let result = pc
                .inner_mut()
                .get(GetRequest {
                    id: new_project_task.clone().id.clone(),
                })
                .await;

            assert!(result.is_err());
            let error = result.err().unwrap();
            let expected = Status::not_found("Project task not found");
            assert_eq!(error.code(), expected.code());
            assert_eq!(error.message(), expected.message());

            Ok(())
        });

        Ok(())
    }
}
