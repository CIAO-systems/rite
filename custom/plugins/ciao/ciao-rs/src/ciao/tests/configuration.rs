use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};

use crate::ciao::ClientManager;

#[tokio::test]
#[ignore]
async fn test_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;
    let values = manager.configuration_client.get_values(None).await?;
    for entry in &values.entries {
        println!("{} = {}", entry.0, entry.1);
    }

    assert_eq!(
        values.entries.get("api_key"),
        Some(&String::from("abcdef123456"))
    );
    assert_eq!(
        values.entries.get("max_connections"),
        Some(&String::from("100"))
    );
    assert_eq!(
        values.entries.get("database_url"),
        Some(&String::from("postgresql://localhost:5432/mydb"))
    );
    assert_eq!(
        values.entries.get("debug_mode"),
        Some(&String::from("true"))
    );
    assert_eq!(values.entries.get("cache_ttl"), Some(&String::from("3600")));
    Ok(())
}
