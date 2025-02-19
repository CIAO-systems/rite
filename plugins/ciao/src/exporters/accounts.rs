use ciao_rs::ciao::{
    accounts::{Account, CreateRequest},
    common::{Address, Image, Name},
};
use export::Exporter;
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
        // 1. Establich connection to gRPC server
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
    let updated_at = match get_timestamp(record, "avatar.updatedAt") {
        Ok(updated_at) => Some(updated_at),
        Err(e) => {
            eprintln!("{e}");
            log::error!("{e}");
            None
        }
    };

    if id.is_some() || updated_at.is_some() {
        Some(Image {
            id: id
                .map(|field| field.value().to_string())
                .unwrap_or_default(),
            updated_at,
        })
    } else {
        None
    }
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
mod tests {
    use model::{field::add_field, record::Record, value::Value, BoxedError};

    use crate::model::add_timestamp_parse;

    use super::*;

    #[test]
    fn test_name_all_fields_present() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "name.first",
            Value::String("John".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "name.middle",
            Value::String("Michael".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "name.last",
            Value::String("Doe".to_string()),
        );

        let name = name_from_record(&record);
        assert!(name.is_some());
        let name = name.unwrap();
        assert_eq!(name.first, "John");
        assert_eq!(name.middle, "Michael");
        assert_eq!(name.last, "Doe");
    }

    #[test]
    fn test_name_some_fields_missing() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "name.first",
            Value::String("Jane".to_string()),
        );
        // Middle name is missing
        add_field(
            record.fields_as_mut(),
            "name.last",
            Value::String("Smith".to_string()),
        );

        let name = name_from_record(&record);
        assert!(name.is_some());
        let name = name.unwrap();
        assert_eq!(name.first, "Jane");
        assert_eq!(name.middle, "");
        assert_eq!(name.last, "Smith");
    }

    #[test]
    fn test_name_all_fields_missing() {
        let record = Record::new();

        let name = name_from_record(&record);
        assert!(name.is_none());
    }

    #[test]
    fn test_name_only_middle_name_present() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "name.middle",
            Value::String("Alexander".to_string()),
        );

        let name = name_from_record(&record);
        assert!(name.is_some());
        let name = name.unwrap();
        assert_eq!(name.first, "");
        assert_eq!(name.middle, "Alexander");
        assert_eq!(name.last, "");
    }

    #[test]
    fn test_address_all_fields_present() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "address.city",
            Value::String("Springfield".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.postalCode",
            Value::String("12345".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.addressLine1",
            Value::String("742 Evergreen Terrace".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.addressLine2",
            Value::String("Apt 1".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.regionCode",
            Value::String("US-IL".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.state",
            Value::String("Illinois".to_string()),
        );

        let address = address_from_record(&record);
        assert!(address.is_some());
        let address = address.unwrap();
        assert_eq!(address.city, "Springfield");
        assert_eq!(address.postal_code, "12345");
        assert_eq!(address.address_line_1, "742 Evergreen Terrace");
        assert_eq!(address.address_line_2, "Apt 1");
        assert_eq!(address.region_code, "US-IL");
        assert_eq!(address.state, "Illinois");
    }

    #[test]
    fn test_address_some_fields_missing() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "address.city",
            Value::String("Gotham".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.postalCode",
            Value::String("54321".to_string()),
        );
        // addressLine1 and addressLine2 are missing
        add_field(
            record.fields_as_mut(),
            "address.regionCode",
            Value::String("US-NY".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.state",
            Value::String("New York".to_string()),
        );

        let address = address_from_record(&record);
        assert!(address.is_some());
        let address = address.unwrap();
        assert_eq!(address.city, "Gotham");
        assert_eq!(address.postal_code, "54321");
        assert_eq!(address.address_line_1, "");
        assert_eq!(address.address_line_2, "");
        assert_eq!(address.region_code, "US-NY");
        assert_eq!(address.state, "New York");
    }

    #[test]
    fn test_address_all_fields_missing() {
        let record = Record::new();

        let address = address_from_record(&record);
        assert!(address.is_none());
    }

    #[test]
    fn test_address_only_city_present() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "address.city",
            Value::String("Metropolis".to_string()),
        );

        let address = address_from_record(&record);
        assert!(address.is_some());
        let address = address.unwrap();
        assert_eq!(address.city, "Metropolis");
        assert_eq!(address.postal_code, "");
        assert_eq!(address.address_line_1, "");
        assert_eq!(address.address_line_2, "");
        assert_eq!(address.region_code, "");
        assert_eq!(address.state, "");
    }

    #[test]
    fn test_avatar_only_id_present() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "avatar.id",
            Value::String("67890".to_string()),
        );

        let avatar = avatar_from_record(&record);
        assert!(avatar.is_some());
        let avatar = avatar.unwrap();
        assert_eq!(avatar.id, "67890");
        assert_eq!(avatar.updated_at, None);
    }

    #[test]
    fn test_avatar_only_updated_at_present() -> Result<(), BoxedError> {
        let mut record = Record::new();
        add_timestamp_parse(
            record.fields_as_mut(),
            "avatar.updatedAt",
            "2025-02-12 08:00",
            "%Y-%m-%d %H:%M",
        )?;

        let avatar = avatar_from_record(&record);
        assert!(avatar.is_some());
        let avatar = avatar.unwrap();
        assert_eq!(avatar.id, "");

        let updated_at = avatar.updated_at.unwrap();
        assert_eq!(updated_at.time_utc.unwrap().seconds, 1739347200);
        assert_eq!(updated_at.time_utc.unwrap().nanos, 0);
        assert_eq!(updated_at.time_zone, "Europe/Berlin");

        Ok(())
    }

    #[test]
    fn test_avatar_all_fields_missing() {
        let record = Record::new();

        let avatar = avatar_from_record(&record);
        assert!(avatar.is_none());
    }

    #[test]
    fn test_avatar_invalid_timestamp() {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "avatar.id",
            Value::String("12345".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "avatar.updateAt",
            Value::String("invalid".to_string()),
        ); // Invalid timestamp

        let avatar = avatar_from_record(&record);
        assert!(avatar.is_some());
        let avatar = avatar.unwrap();
        assert_eq!(avatar.id, "12345");
        assert_eq!(avatar.updated_at, None);
    }

    #[test]
    fn test_all_fields_present() -> Result<(), BoxedError> {
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "id",
            Value::String("98765".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "name.first",
            Value::String("Alice".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "name.middle",
            Value::String("B".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "name.last",
            Value::String("Smith".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.city",
            Value::String("Wonderland".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.postalCode",
            Value::String("54321".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.addressLine1",
            Value::String("123 Main St".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.regionCode",
            Value::String("US-CA".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "address.state",
            Value::String("California".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "avatar.id",
            Value::String("avatar123".to_string()),
        );

        add_timestamp_parse(
            record.fields_as_mut(),
            "avatar.updatedAt",
            "2025-02-12 08:00",
            "%Y-%m-%d %H:%M",
        )?;

        add_field(
            record.fields_as_mut(),
            "email",
            Value::String("alice@example.com".to_string()),
        );

        let account = account_from_record(&record);

        assert_eq!(account.id, "98765");
        assert_eq!(account.email, "alice@example.com");

        let name = account.name.unwrap();
        assert_eq!(name.first, "Alice");
        assert_eq!(name.middle, "B");
        assert_eq!(name.last, "Smith");

        let address = account.address.unwrap();
        assert_eq!(address.city, "Wonderland");
        assert_eq!(address.postal_code, "54321");
        assert_eq!(address.address_line_1, "123 Main St");
        assert_eq!(address.region_code, "US-CA");
        assert_eq!(address.state, "California");

        let avatar = account.avatar.unwrap();
        assert_eq!(avatar.id, "avatar123");
        assert!(avatar.updated_at.is_some());
        let updated_at = avatar.updated_at.unwrap();
        assert_eq!(updated_at.time_utc.unwrap().seconds, 1739347200);
        assert_eq!(updated_at.time_utc.unwrap().nanos, 0);
        assert_eq!(updated_at.time_zone, "Europe/Berlin");

        Ok(())
    }
}
