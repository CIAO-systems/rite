use grpc_utils_rs::grpc::interceptor::Interceptors;

pub mod manager;

#[derive(Debug)]
pub struct DatasetClient;
impl DatasetClient {
    async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }
}
