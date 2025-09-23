use std::{collections::HashMap, error::Error, pin::Pin};

use futures::Stream;
use grpc_utils_rs::interceptors;
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    com::atoss::atc::protobuf::{
        absences_service_server::{AbsencesService, AbsencesServiceServer},
        data_set_service_server::{DataSetService, DataSetServiceServer},
        field::Value,
        Absence, AbsencesRequest, Field, Filter, Record,
    },
    connection::{clients::manager::ClientManager, interceptor::ATCClientInterceptor},
};
pub struct MockDataSetService;
pub struct MockAbsenceService;

#[tonic::async_trait]
impl DataSetService for MockDataSetService {
    type getStream = Pin<Box<dyn Stream<Item = Result<Record, Status>> + Send + 'static>>;

    async fn get(&self, request: Request<Filter>) -> Result<Response<Self::getStream>, Status> {
        println!("{:?}", request);

        // Create fake records based on filter
        let mut fields = HashMap::new();
        fields.insert(
            String::from("table"),
            Field {
                name: "table".into(),
                value: Some(Value::StringValue(request.into_inner().table.clone())),
            },
        );
        let record = Record { field: fields };

        let records = vec![record];

        // Turn Vec<Record> into a stream of Result<Record, Status>
        let output = tokio_stream::iter(records.into_iter().map(|r| Ok(r)));

        Ok(Response::new(Box::pin(output) as Self::getStream))
    }
}

#[tonic::async_trait]
impl AbsencesService for MockAbsenceService {
    type getSingleDayAbsencesStream =
        Pin<Box<dyn Stream<Item = Result<Absence, Status>> + Send + 'static>>;

    async fn get_single_day_absences(
        &self,
        _request: Request<AbsencesRequest>,
    ) -> Result<Response<Self::getSingleDayAbsencesStream>, Status> {
        // mock response stream
        let absence = Absence {
            start_date: None,
            end_date: None,
            account_id: 0,
            employee_id: "employee".into(),
            weight_start: 1.0,
            weight_end: 1.0,
            plan_version: 1, // ATC valid version
            remark: "remark".into(),
            application: "application".into(),
            substitute: "substitute".into(),
            state: 0,
            display_token: "display".into(),
            description: "description".into(),
            display_color: 0,
            text_color: 0,
            date: None,
            time: None,
        };
        let absences = vec![absence];

        let output = tokio_stream::iter(absences.into_iter().map(|a| Ok(a)));

        Ok(Response::new(Box::pin(output)))
    }
}

/// Starts a mock gRPC server
pub async fn start_mock_server(port: u32) -> std::net::SocketAddr {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap(); // Force IPv4
    let addr = listener.local_addr().unwrap();

    // Spawn server in the background
    tokio::spawn(async move {
        println!("spawn mock server");
        let result = Server::builder()
            .add_service(DataSetServiceServer::new(MockDataSetService))
            .add_service(AbsencesServiceServer::new(MockAbsenceService))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await;
        println!("{:?}", result);
        assert!(result.is_ok());
        result.unwrap();
    });

    addr
}

pub async fn get_mock_client_manager(port: u32) -> Result<ClientManager, Box<dyn Error>> {
    let addr = start_mock_server(port).await;

    ClientManager::new(
        &format!("http://{}", addr),
        interceptors!(ATCClientInterceptor::new(
            &String::from("auth_token"),
            &String::from("user"),
            &String::from("password"),
        )),
    )
    .await
}
