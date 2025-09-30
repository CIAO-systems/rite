use ciao_rs::ciao::{
    accounts::{Account, CreateRequest},
    common::{Address, Image, Name},
};
use model::export::Exporter;
use model::Initializable;

use crate::{connection::CiaoConnection, model::get_timestamp};

pub struct Accounts {
    config: Option<model::xml::config::Configuration>,
    connection: Option<CiaoConnection>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            config: None,
            connection: None,
        }
    }
}

impl Initializable for Accounts {
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

impl Exporter for Accounts {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut connection) = self.connection {
            if let Some(ref mut client) = connection.client {
                // 2. Retrieve the client that fits the need
                let mut service_client = &mut client.account_client;
                if let Some(ref runtime) = connection.runtime {
                    // 3. Use the connection tokio runtime to call a service
                    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async {
                        create_account(&mut service_client, record).await?;
                        Ok(())
                    });
                    result?
                }
            }
        }
        Ok(())
    }
}

async fn create_account(
    service_client: &mut ciao_rs::ciao::clients::accounts::AccountClient,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    let password = record
        .field_by_name("password")
        .map(|field| field.value().to_string())
        .unwrap_or_default();

    let request = CreateRequest {
        account: Some(account_from_record(record)),
        password,
    };

    let response = service_client
        .inner_mut()
        .create(request)
        .await?
        .into_inner();
    log::info!("Account {:?} created", response.account);
    Ok(())
}

fn account_from_record(record: &model::record::Record) -> ciao_rs::ciao::accounts::Account {
    Account {
        id: record
            .field_by_name("id")
            .map(|field| field.value().to_string())
            .unwrap_or_default(),
        name: name_from_record(record),
        address: address_from_record(record),
        avatar: avatar_from_record(record),
        email: record
            .field_by_name("email")
            .map(|field| field.value().to_string())
            .unwrap_or_default(),
    }
}

fn avatar_from_record(record: &model::record::Record) -> Option<ciao_rs::ciao::common::Image> {
    let id = record.field_by_name("avatar.id");
    if id.is_some() {
        let updated_at = match get_timestamp(record, "avatar.updatedAt") {
            Ok(updated_at) => Some(updated_at),
            Err(e) => {
                eprintln!("{e}");
                log::error!("{e}");
                None
            }
        };

        if updated_at.is_some() {
            return Some(Image {
                id: id
                    .map(|field| field.value().to_string())
                    .unwrap_or_default(),
                updated_at,
            });
        }
    }
    None
}

fn address_from_record(record: &model::record::Record) -> Option<ciao_rs::ciao::common::Address> {
    let city = record.field_by_name("address.city");
    let postal_code = record.field_by_name("address.postalCode");
    let address_line_1 = record.field_by_name("address.addressLine1");
    let address_line_2 = record.field_by_name("address.addressLine2");
    let region_code = record.field_by_name("address.regionCode");
    let state = record.field_by_name("address.state");

    if city.is_some()
        || postal_code.is_some()
        || address_line_1.is_some()
        || address_line_2.is_some()
        || region_code.is_some()
        || state.is_some()
    {
        Some(Address {
            city: city
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            postal_code: postal_code
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            address_line_1: address_line_1
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            address_line_2: address_line_2
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            region_code: region_code
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            state: state
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
        })
    } else {
        None
    }
}

fn name_from_record(record: &model::record::Record) -> Option<ciao_rs::ciao::common::Name> {
    let first = record.field_by_name("name.first");
    let middle = record.field_by_name("name.middle");
    let last = record.field_by_name("name.last");

    if first.is_some() || middle.is_some() || last.is_some() {
        Some(Name {
            first: first
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            middle: middle
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            last: last
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
