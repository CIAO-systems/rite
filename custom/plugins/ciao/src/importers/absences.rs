use ciao_rs::ciao::{
    clients::time_tracking::absences::AbsenceClient, time_tracking::absences::Absence,
};
use futures::StreamExt;
use import::{Importer, RecordHandler};
use model::{xml::config::Configuration, BoxedError, Initializable};

use crate::{
    config::{get_config_time_range, get_config_values},
    connection::CiaoConnection,
};

pub struct Absences {
    config: Option<model::xml::config::Configuration>,
}

impl Absences {
    pub fn new() -> Self {
        Absences { config: None }
    }
}

impl Initializable for Absences {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Absences {
    fn read(
        &mut self,
        handler: &mut dyn import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establich connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.absence_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_absences(&self.config, service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }
        Ok(())
    }
}

async fn list_absences(
    config: &Option<Configuration>,
    mut service_client: AbsenceClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), BoxedError> {
    let mut stream = service_client
        .inner_mut()
        .list(ciao_rs::ciao::time_tracking::absences::ListRequest {
            time_range: get_config_time_range(config, "filter.timeRange"),
            user_ids: get_config_values(config, "filter.userIds"),
            time_type_ids: get_config_values(config, "filter.timeTypeIds"),
        })
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for absence in r.absences {
                    handle_absence(&absence, handler)?;
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn handle_absence(_absence: &Absence, _handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
    Ok(())
}
