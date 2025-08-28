use ciao_rs::ciao::{
    accounts::badges::{Badge, ListRequest},
    clients::accounts::badges::BadgeClient,
};
use futures::StreamExt;
use model::import::{Importer, RecordHandler};
use model::{
    field::add_field,
    record::Record,
    value::Value,
    xml::config::{get_config_value, Configuration},
    BoxedError, Initializable,
};

use crate::connection::CiaoConnection;

const CFG_USER_ID: &str = "filter.userId";

pub struct Badges {
    config: Option<model::xml::config::Configuration>,
}

impl Badges {
    pub fn new() -> Self {
        Badges { config: None }
    }
}

impl Initializable for Badges {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Badges {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establish connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.badge_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_badges(&self.config, service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }
        Ok(())
    }
}

async fn list_badges(
    config: &Option<Configuration>,
    mut service_client: BadgeClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), BoxedError> {
    let mut stream = service_client
        .inner_mut()
        .list(ListRequest {
            user_id: get_config_value(config, CFG_USER_ID),
        })
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for badge in r.badges {
                    handle_badge(&badge, handler)?;
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn handle_badge(badge: &Badge, handler: &mut dyn RecordHandler) -> Result<(), BoxedError> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    // string id = 1;
    add_field(fields, "id", Value::String(badge.id.clone()));

    // string user_id = 2;
    add_field(fields, "userId", Value::String(badge.user_id.clone()));

    // string external_id = 3;
    add_field(
        fields,
        "externalId",
        Value::String(badge.external_id.clone()),
    );

    // optional string description = 4;
    if let Some(ref description) = badge.description {
        add_field(fields, "description", Value::String(description.clone()));
    }

    handler.handle_record(&mut record)?;

    Ok(())
}
