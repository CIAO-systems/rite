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
pub mod tests {
    use std::{collections::HashMap, time::Duration};

    use tokio_stream::StreamExt;

    use crate::{
        com::atoss::atc::protobuf::{field::Value, AbsencesRequest, Filter},
        connection::clients::manager::tests::mocks::get_mock_client_manager,
    };

    pub mod mocks;

    #[tokio::test]
    async fn test_new() {
        let cm = get_mock_client_manager(50052).await;
        assert!(cm.is_ok());

        let wait_millis = 50;
        println!("wait for {wait_millis} milliseconds...");
        tokio::time::sleep(Duration::from_millis(wait_millis)).await;

        let mut cm = cm.unwrap();
        let request = Filter {
            table: "table".into(),
            parameter_meta_data: HashMap::new(),
        };

        let mut stream = cm
            .dataset_client
            .inner_mut()
            .get(request)
            .await
            .unwrap()
            .into_inner();
        while let Some(response) = stream.next().await {
            assert!(response.is_ok());
            let response = response.unwrap();
            let field = response.field.get("table");
            assert!(field.is_some());
            let field = field.unwrap();
            let value = field.value.clone();
            assert!(value.is_some());
            let value = value.unwrap();
            println!("{:?}", value);
            assert_eq!(value, Value::StringValue("table".into()));
        }

        let request = AbsencesRequest {
            employee_ids: Vec::new(),
            start_date: None,
            end_date: None,
            account_ids: Vec::new(),
            plan_version: 0,
            options: None,
        };

        let mut stream = cm
            .absences_client
            .inner_mut()
            .get_single_day_absences(request)
            .await
            .unwrap()
            .into_inner();
        while let Some(response) = stream.next().await {
            assert!(response.is_ok());
            let response = response.unwrap();
            assert_eq!(response.employee_id, "employee");
        }
    }
}
