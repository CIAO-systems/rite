use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};

use crate::ciao::ClientManager;

#[tokio::test]
#[ignore]
async fn test_authentication_email() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;
    let login_result = manager
        .authentication_client
        .login_email("demo.user@ciao-systems.com", "secret")
        .await?;

    println!("{:?}", login_result.account);

    let account = login_result.account.unwrap();
    assert_eq!("number-1-employee", &account.id);
    let name = &account.name.unwrap();
    assert_eq!("Demo", name.first);
    assert_eq!("User", name.last);
    Ok(())
}
