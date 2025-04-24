use clients::{
    accounts::AccountClient,
    authentication::AuthenticationClient,
    configuration::ConfigurationClient,
    devices::DeviceClient,
    time_tracking::{cost_centers::CostCenterClient, project_tasks::ProjectTaskClient, projects::ProjectClient, time_type::TimeTypeClient, TimeTrackingClient},
};
use grpc_utils_rs::grpc::interceptor::Interceptors;

/// Common messages like Timestamp or Color
pub mod common;

/// Time tracking messages and gRPC service stubs
pub mod time_tracking;

/// Device messages and gRPC service stubs
pub mod devices;

/// Account messages and services
pub mod accounts;

/// Clients for the services
pub mod clients;

/// Core services
pub mod core;


#[derive(Debug)]
pub struct ClientManager {
    pub account_client: AccountClient,
    pub authentication_client: AuthenticationClient,
    pub configuration_client: ConfigurationClient,
    pub device_client: DeviceClient,
    pub project_client: ProjectClient,
    pub project_task_client: ProjectTaskClient,
    pub time_tracking_client: TimeTrackingClient,
    pub time_type_client: TimeTypeClient,
    pub cost_center_client: CostCenterClient,
}

impl ClientManager {
    pub async fn new(
        url: &str,
        interceptors: Interceptors,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            account_client: AccountClient::new(url, interceptors.clone()).await?,
            authentication_client: AuthenticationClient::new(url, interceptors.clone()).await?,
            configuration_client: ConfigurationClient::new(url, interceptors.clone()).await?,
            device_client: DeviceClient::new(url, interceptors.clone()).await?,
            project_client: ProjectClient::new(url, interceptors.clone()).await?,
            project_task_client: ProjectTaskClient::new(url, interceptors.clone()).await?,
            time_tracking_client: TimeTrackingClient::new(url, interceptors.clone()).await?,
            time_type_client: TimeTypeClient::new(url, interceptors.clone()).await?,
            cost_center_client: CostCenterClient::new(url, interceptors.clone()).await?,
        })
    }
}

#[cfg(test)]
mod tests;
