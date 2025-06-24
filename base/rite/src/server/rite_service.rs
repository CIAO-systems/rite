use std::io::Cursor;

use crate::proto::rite::v1::{rite_service_server::RiteService, ProcessRequest, ProcessResponse};
use tonic::{Request, Response, Status};
use zip::ZipArchive;

#[derive(Debug, Default)]
pub struct RiteServiceImpl;

#[tonic::async_trait]
impl RiteService for RiteServiceImpl {
    async fn process(
        &self,
        request: Request<ProcessRequest>,
    ) -> Result<Response<ProcessResponse>, Status> {
        let request = request.into_inner();
        println!("Received process request: {:?}", request);

        // Extract the zipped_configuration bytes
        let data = request.zipped_configuration;

        // Use a Cursor to allow ZipArchive to read from the byte array
        let reader = Cursor::new(data);

        // Try to open the zip archive
        let mut zip = ZipArchive::new(reader)
            .map_err(|e| Status::invalid_argument(format!("Failed to read zip archive: {}", e)))?;

        // Collect file names
        let mut files = Vec::new();
        for i in 0..zip.len() {
            let file = zip
                .by_index(i)
                .map_err(|e| Status::internal(format!("Failed to access file in zip: {}", e)))?;
            files.push(file.name().to_string());
        }

        // For now, returning an empty response
        let response = ProcessResponse { files };

        Ok(Response::new(response))
    }
}
