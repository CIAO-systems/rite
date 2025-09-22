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

#[cfg(test)]
mod tests {
    use grpc_utils_rs::interceptors;
    use tokio::net::TcpListener;
    use tonic::transport::Server;

    use crate::{
        com::atoss::atc::protobuf::{
            absences_service_server::AbsencesServiceServer,
            data_set_service_server::DataSetServiceServer,
        },
        connection::{
            clients::manager::{
                tests::mocks::{MockAbsenceService, MockDataSetService},
                ClientManager,
            },
            interceptor::ATCClientInterceptor,
        },
    };

    mod mocks;

    #[tokio::test]
    #[ignore = "FIXME Does not connect to mock server: tonic::transport::Error(Transport, InvalidDnsNameError)"]
    async fn test_new() {
        let listener = TcpListener::bind("localhost:50051").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn server in the background
        tokio::spawn(async move {
            Server::builder()
                .add_service(DataSetServiceServer::new(MockDataSetService))
                .add_service(AbsencesServiceServer::new(MockAbsenceService))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
                .unwrap();
        });

        // Wait did not hel
        // let _ = tokio::time::sleep(Duration::from_millis(5000));

        let cm = ClientManager::new(
            &format!("http://{}", addr),
            interceptors!(ATCClientInterceptor::new(
                &String::from("auth_token"),
                &String::from("user"),
                &String::from("password"),
            )),
        )
        .await;
        println!("{:?}", cm);
        assert!(cm.is_err());
    }
}
