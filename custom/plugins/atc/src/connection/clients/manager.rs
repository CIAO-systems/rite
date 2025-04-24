use grpc_utils_rs::grpc::interceptor::Interceptors;

use super::DataSetClient;

#[derive(Debug)]
pub struct ClientManager {
    pub dataset_client: DataSetClient,
}

impl ClientManager {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            dataset_client: DataSetClient::new(url, interceptors.clone()).await?,
        })
    }
}
