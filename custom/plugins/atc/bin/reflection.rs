pub mod grpc_reflection;

use prost::Message;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::Request;
use tonic::Streaming;

use grpc_reflection::v1alpha::{
    server_reflection_client::ServerReflectionClient, server_reflection_request::MessageRequest,
    ServerReflectionRequest, ServerReflectionResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the gRPC server URL from the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <grpc-server-url>", args[0]);
        std::process::exit(1);
    }
    let grpc_server_url = &args[1];

    let tls = tonic::transport::ClientTlsConfig::new().with_native_roots();
    // Create an endpoint
    let endpoint = if grpc_server_url.starts_with("https") {
        Endpoint::from_shared(grpc_server_url.to_string())?.tls_config(tls)?
        // Enable native TLS
    } else {
        Endpoint::from_shared(grpc_server_url.to_string())?
    };

    // Connect to the provided gRPC server URL with native TLS
    let channel = endpoint.connect().await?;
    let mut client = ServerReflectionClient::new(channel);

    // Step 1: Get all services first
    let services = list_services(&mut client).await?;

    // Step 2: Fetch messages for each service separately
    for service in services {
        let messages = fetch_messages_for_service(&mut client, &service).await?;
        println!("Service: {}", service);
        println!("  Messages used:");
        for msg in messages {
            println!("    - {}", msg);
        }
    }

    Ok(())
}

/// Retrieve all available gRPC services from reflection
async fn list_services(
    client: &mut ServerReflectionClient<Channel>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(10);

    // Send ListServices request
    let request = ServerReflectionRequest {
        host: "".to_string(),
        message_request: Some(MessageRequest::ListServices("".to_string())),
    };
    tx.send(request).await.unwrap();

    let request_stream = Request::new(ReceiverStream::new(rx));
    let mut response_stream: Streaming<ServerReflectionResponse> = client
        .server_reflection_info(request_stream)
        .await?
        .into_inner();

    let mut service_names = Vec::new();

    // Process responses
    while let Some(response) = response_stream.message().await? {
        if let Some(grpc_reflection::v1alpha::server_reflection_response::MessageResponse::ListServicesResponse(service_list)) = response.message_response {
            for service in service_list.service {
                service_names.push(service.name);
            }
        }
    }

    Ok(service_names)
}

/// Fetch all Protobuf messages used in a given service
async fn fetch_messages_for_service(
    client: &mut ServerReflectionClient<Channel>,
    service_name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(10);

    // Send a request to get the file descriptor for the service
    let request = ServerReflectionRequest {
        host: "".to_string(),
        message_request: Some(MessageRequest::FileContainingSymbol(
            service_name.to_string(),
        )),
    };
    tx.send(request).await.unwrap();

    let request_stream = Request::new(ReceiverStream::new(rx));
    let mut response_stream = client
        .server_reflection_info(request_stream)
        .await?
        .into_inner();

    let mut message_types = Vec::new();

    // Process the response stream
    while let Some(response) = response_stream.message().await? {
        if let Some(grpc_reflection::v1alpha::server_reflection_response::MessageResponse::FileDescriptorResponse(descriptor_response)) = response.message_response {
            for file_descriptor_bytes in descriptor_response.file_descriptor_proto {
                // Parse the file descriptor set
                let descriptor = prost_types::FileDescriptorProto::decode(&*file_descriptor_bytes)?;
                message_types.extend(descriptor.message_type.into_iter().map(|m| m.name.unwrap_or_default()));
            }
        }
    }

    Ok(message_types)
}
