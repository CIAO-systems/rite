use grpc_utils_rs::grpc::interceptor::Interceptors;

use crate::connection::clients::AbsencesClient;

use super::DataSetClient;

#[derive(Debug)]
pub struct ClientManager {
    pub dataset_client: DataSetClient,
    pub absences_client: AbsencesClient,
}

impl ClientManager {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            dataset_client: DataSetClient::new(url, interceptors.clone()).await?,
            absences_client: AbsencesClient::new(url, interceptors.clone()).await?,
        })
    }
}
