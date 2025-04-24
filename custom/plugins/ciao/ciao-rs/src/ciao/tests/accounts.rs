use grpc_utils_rs::{grpc::interceptor::APIKeyClientInterceptor, interceptors};

use crate::ciao::{
    accounts::{Account, CreateRequest},
    common::Name,
    ClientManager,
};

#[tokio::test]
#[ignore]
async fn test_account_get() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;
    let response = manager
        .account_client
        .get("550e8400-e29b-41d4-a716-446655440000")
        .await?;

    println!("{:?}", response);

    let account = response.account.unwrap();
    assert_eq!("550e8400-e29b-41d4-a716-446655440000", &account.id);
    let name = &account.name.unwrap();
    assert_eq!("John", name.first);
    assert_eq!("William", name.middle);
    assert_eq!("Doe", name.last);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_account_list() -> Result<(), Box<dyn std::error::Error>> {
    use futures::StreamExt;

    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;
    let mut stream = manager.account_client.list().await?;
    while let Some(account) = stream.next().await {
        match account {
            Ok(account) => println!("{:?}", account),
            Err(e) => eprintln!("Error receiving user: {:?}", e),
        }
    }

    Ok(())
}

#[tokio::test]
#[ignore = "for manual testing"]
async fn test_account_inner() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ClientManager::new(
        "http://localhost:50051",
        interceptors!(APIKeyClientInterceptor::new(
            "top-secret-api-key".to_string()
        )),
    )
    .await?;
    let client = manager.account_client.inner_mut();
    let request = CreateRequest {
        account: Some(Account {
            id: String::from("new-id"),
            name: Some(Name {
                first: String::from("first"),
                middle: String::from("middle"),
                last: String::from("last"),
            }),
            address: None,
            avatar: None,
            email: "".to_string(),
        }),
        password: String::from("super-scret"),
    };
    // Step 1: Create a new account
    let created = client.create(request).await?.into_inner();
    println!("{:?}", created);

    // Step 2: Read the account
    let response = manager.account_client.get("new-id").await?;
    assert!(response.account.is_some());
    if let Some(account) = response.account {
        // Step 3: Check, if it is the same
        assert_eq!("new-id", account.id);
        assert!(account.name.is_some());
        if let Some(name) = account.name {
            assert_eq!("first", name.first);
            assert_eq!("middle", name.middle);
            assert_eq!("last", name.last);
        }
    }
    Ok(())
}
