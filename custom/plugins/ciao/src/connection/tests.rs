use model::xml::config::Configuration;

use crate::{
    config::{CFG_API_KEY, CFG_URL},
    connection::CiaoConnection,
};

#[test]
fn test_connect_none() {
    let connection = CiaoConnection::connect(&None);
    assert!(connection.is_err_and(|e| e.to_string() == "Configuration incomplete"));
}

#[test]
fn test_connect_some() {
    let mut config = Configuration::new();
    config.insert_str(CFG_URL, "incorrect://localhost:50051");
    config.insert_str(CFG_API_KEY, "some-api-key");
    let connection = CiaoConnection::connect(&Some(config));
    println!("{:?}", connection);
    assert!(connection.is_err_and(|e| {
        println!("{:?}", e.to_string());
        e.to_string() == "gRPC transport error: Failed to connect to gRPC server on incorrect://localhost:50051. Original error: transport error"
    }));
}

mod manual;