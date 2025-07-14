use ciao_rs::ciao::{accounts::badges::Badge, clients::accounts::badges::BadgeClient};
use export::Exporter;
use model::{BoxedError, Initializable};
use uuid::Uuid;

use crate::{
    connection::CiaoConnection,
    model::{get_mandatory_string, get_optional_string},
};

pub struct Badges {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl Badges {
    pub(crate) fn new() -> Self {
        Self {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for Badges {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        self.config = config;
        // 1. Establish connection to gRPC server
        self.connection = Some(CiaoConnection::connect(&self.config)?);

        Ok(())
    }
}

impl Exporter for Badges {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.badge_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_badge(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }

        Ok(())
    }
}

async fn create_badge(
    service_client: &mut BadgeClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let badge = Some(badge_from_record(record)?);

    service_client
        .inner_mut()
        .create(ciao_rs::ciao::accounts::badges::CreateRequest { badge: badge })
        .await?;

    Ok(())
}

fn badge_from_record(record: &model::record::Record) -> Result<Badge, BoxedError> {
    let id = Uuid::new_v4().to_string();
    let user_id = get_mandatory_string(record, "userId")?;
    let external_id = get_optional_string(record, "externalId").unwrap_or_default();
    let description = get_optional_string(record, "description");
    Ok(Badge {
        id,
        user_id,
        external_id,
        description,
    })
}
