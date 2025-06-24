use dotenv::dotenv;
use log::info;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;

use crate::rite::v1::{
    rite_service_server::{RiteService, RiteServiceServer},
    ProcessRequest, ProcessResponse,
};

// Include the generated protobuf code
pub mod rite {
    pub mod v1 {
        tonic::include_proto!("rite.v1");
    }
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    log4rs::init_file("log4rs.yaml", Default::default())?;

    let addr = "0.0.0.0:50051".parse()?;
    let rite_service = RiteServiceImpl::default();

    info!(
        "Rust Import/Transform/Export (Server listening on {})",
        addr
    );

    Server::builder()
        .add_service(RiteServiceServer::new(rite_service))
        .add_service(reflection_v1()?)
        .add_service(reflection_v1alpha()?)
        .serve(addr)
        .await?;

    Ok(())
}

fn reflection_v1() -> Result<
    tonic_reflection::server::v1::ServerReflectionServer<
        impl tonic_reflection::server::v1::ServerReflection,
    >,
    tonic_reflection::server::Error,
> {
    Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/service_descriptor.bin"
        )))
        .build_v1()
}

fn reflection_v1alpha() -> Result<
    tonic_reflection::server::v1alpha::ServerReflectionServer<
        impl tonic_reflection::server::v1alpha::ServerReflection,
    >,
    tonic_reflection::server::Error,
> {
    Builder::configure()
        .register_encoded_file_descriptor_set(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/service_descriptor.bin"
        )))
        .build_v1alpha()
}
