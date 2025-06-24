use std::env;

use tonic::{service::Interceptor, Request, Status};

const API_KEY_HEADER: &str = "x-api-key";
const API_KEY_ENV: &str = "GRPC_API_KEY";

fn check_api_key<T>(req: &Request<T>) -> bool {
    // Get the API key from the environment (could be replaced with more complex logic later)
    let expected_key = match env::var(API_KEY_ENV) {
        Ok(val) => val,
        Err(_) => return false,
    };
    // Extract the API key from metadata
    match req.metadata().get(API_KEY_HEADER) {
        Some(val) => val == expected_key.as_str(),
        None => false,
    }
}

pub fn api_key_interceptor() -> impl Interceptor + Clone {
    move |req: Request<()>| {
        if check_api_key(&req) {
            Ok(req)
        } else {
            Err(Status::unauthenticated("Invalid or missing API key"))
        }
    }
}
