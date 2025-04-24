use crate::ciao::ClientManager;
use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};

#[tokio::test]
#[ignore = "for manual testing"]
async fn test_device_service() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;
    let response = manager
        .device_client
        .get_device_configuration("device_id")
        .await?;

    // TODO: implement, when service is implemented in the demo server
    println!("{:?}", response);

    Ok(())
}
