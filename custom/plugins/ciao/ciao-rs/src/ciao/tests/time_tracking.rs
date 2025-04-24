use crate::ciao::ClientManager;
use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};

#[tokio::test]
#[ignore]
async fn test_time_tracking_list() -> Result<(), Box<dyn std::error::Error>> {
    use futures::StreamExt;

    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;

    // TODO: implement, when service is implemented in the demo server
    let mut stream = manager
        .time_tracking_client
        .list(None, None, None, None)
        .await?;
    while let Some(entry) = stream.next().await {
        match entry {
            Ok(time_type) => println!("{:?}", time_type),
            Err(e) => eprintln!("Error receiving user: {:?}", e),
        }
    }

    Ok(())
}
