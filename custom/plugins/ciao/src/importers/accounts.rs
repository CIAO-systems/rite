use ciao_rs::ciao::accounts::ListRequest;
use futures::StreamExt;
use model::import::{Importer, RecordHandler};
use model::{field::add_field, record::Record, value::Value, BoxedError, Initializable};

use crate::connection::CiaoConnection;

pub struct Accounts {
    config: Option<model::xml::config::Configuration>,
}

impl Accounts {
    pub fn new() -> Self {
        Self { config: None }
    }
}

impl Initializable for Accounts {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        self.config = config;
        Ok(())
    }
}

impl Importer for Accounts {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Establish connection to gRPC server
        let connection = CiaoConnection::connect(&self.config)?;
        if let Some(client) = connection.client {
            // 2. Retrieve the client that fits the need
            let service_client = client.account_client;
            if let Some(runtime) = connection.runtime {
                // 3. Use the connection tokio runtime to call a service
                let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                    list_accounts(service_client, handler).await?;
                    Ok(())
                });
                result?
            }
        }

        Ok(())
    }
}

async fn list_accounts(
    mut service_client: ciao_rs::ciao::clients::accounts::AccountClient,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = service_client
        .inner_mut()
        .list(ListRequest {})
        .await?
        .into_inner();
    while let Some(response) = stream.next().await {
        match response {
            Ok(r) => {
                for account in r.accounts {
                    handle_account(&account, handler)?;
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn handle_account(
    account: &ciao_rs::ciao::accounts::Account,
    handler: &mut dyn RecordHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::String(account.id.clone()));
    add_field(fields, "email", Value::String(account.email.clone()));
    if let Some(ref name) = account.name {
        add_field(fields, "name.first", Value::String(name.first.clone()));
        add_field(fields, "name.middle", Value::String(name.middle.clone()));
        add_field(fields, "name.last", Value::String(name.last.clone()));
    }

    if let Some(ref address) = account.address {
        add_field(fields, "address.city", Value::String(address.city.clone()));
        add_field(
            fields,
            "address.postal_code",
            Value::String(address.postal_code.clone()),
        );
        add_field(
            fields,
            "address.address_line_1",
            Value::String(address.address_line_1.clone()),
        );
        add_field(
            fields,
            "address.address_line_2",
            Value::String(address.address_line_2.clone()),
        );
        add_field(
            fields,
            "address.region_code",
            Value::String(address.region_code.clone()),
        );
        add_field(
            fields,
            "address.state",
            Value::String(address.state.clone()),
        );
    }

    if let Some(ref avatar) = account.avatar {
        add_field(fields, "avatar.id", Value::String(avatar.id.clone()));
        if let Some(ref updated_at) = avatar.updated_at {
            add_field(
                fields,
                "avatar.updatedAt.timeUtc",
                Value::I64(updated_at.time_utc.map(|ts| ts.seconds).unwrap_or(0)),
            );
            add_field(
                fields,
                "avatar.updatedAt.timeZone",
                Value::String(updated_at.time_zone.clone()),
            );
        }
    }
    handler.handle_record(&mut record)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use model::import::{handlers::CollectingRecordHandler, Importer};
    use model::{xml::config::Configuration, Initializable};

    use crate::importers::accounts::Accounts;

    #[test]
    #[ignore = "for manual testing"]
    fn test_accounts_importer() -> Result<(), Box<dyn std::error::Error>> {
        let mut importer = Accounts::new();
        let mut config = Configuration::new();
        config.insert("url".to_string(), "http://localhost:50051".to_string());
        config.insert("api-key".to_string(), "top-secret-api-key".to_string());

        importer.init(Some(config))?;
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        importer.read(&mut handler)?;

        assert!(records.len() > 0);
        for account in records {
            println!("{:?}", account);
            assert!(account.field_by_name("id").is_some());
            assert!(account.field_by_name("email").is_some());
        }
        Ok(())
    }
}
