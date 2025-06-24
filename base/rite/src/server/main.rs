use dotenv::dotenv;
use log::info;
use tonic::transport::Server;

use crate::{
    proto::rite::v1::rite_service_server::RiteServiceServer, rite_service::RiteServiceImpl,
};

mod reflection;

// Include the generated protobuf code
pub mod proto;
// Include the service implementation
pub mod rite_service;

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
        .add_service(reflection::v1()?)
        .add_service(reflection::v1alpha()?)
        .serve(addr)
        .await?;

    Ok(())
}
