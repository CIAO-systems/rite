use crate::{
    proto::rite::v1::{rite_service_server::RiteService, ProcessRequest, ProcessResponse},
    rite_service::processor::ServiceProcessor,
};
use tonic::{Request, Response, Status};

pub mod processor;

#[derive(Debug, Default)]
pub struct RiteServiceImpl;

#[tonic::async_trait]
impl RiteService for RiteServiceImpl {
    async fn process(
        &self,
        request: Request<ProcessRequest>,
    ) -> Result<Response<ProcessResponse>, Status> {
        let request = request.into_inner();

        // Extract the zipped_configuration bytes
        let service_processor =
            ServiceProcessor::new(&request.zipped_configuration, request.main_config).map_err(
                |e| Status::invalid_argument(format!("Failed to read zip archive: {}", e)),
            )?;

        let response = service_processor
            .process()
            .map_err(|e| Status::internal(format!("Failed to process configuration: {}", e)))?;

        Ok(Response::new(response))
    }
}
