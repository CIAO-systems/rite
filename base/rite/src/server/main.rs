use dotenv::dotenv;
use log::info;
use tonic::transport::Server;

use crate::{
    protection::api_key_interceptor, proto::rite::v1::rite_service_server::RiteServiceServer,
    rite_service::RiteServiceImpl,
};

mod reflection;

// Include the generated protobuf code
pub mod proto;
// Include the service implementation
pub mod rite_service;
// Include API protection 
pub mod protection;


const GRPC_SERVER_ADDR_ENV: &str = "GRPC_SERVER_ADDR";
const DEFAULT_GRPC_SERVER_ADDR: &str = "[::1]:50051";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    log4rs::init_file("log4rs.yaml", Default::default())?;

    let addr = std::env::var(GRPC_SERVER_ADDR_ENV)
        .unwrap_or_else(|_| DEFAULT_GRPC_SERVER_ADDR.to_string())
        .parse()?;

    info!(
        "Rust Import/Transform/Export - Server (listening on {})",
        addr
    );

    let rite_service = RiteServiceImpl::default();
    Server::builder()
        .add_service(RiteServiceServer::with_interceptor(rite_service, api_key_interceptor()))
        .add_service(reflection::v1()?)
        .add_service(reflection::v1alpha()?)
        .serve(addr)
        .await?;

    Ok(())
}
