use crate::proto::rite::v1::{rite_service_server::RiteService, ProcessRequest, ProcessResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct RiteServiceImpl;

#[tonic::async_trait]
impl RiteService for RiteServiceImpl {
    async fn process(
        &self,
        request: Request<ProcessRequest>,
    ) -> Result<Response<ProcessResponse>, Status> {
        let x = request.into_inner();
        println!("Received process request: {:?}", x);

        // For now, returning an empty response
        let response = ProcessResponse {};

        Ok(Response::new(response))
    }
}
