use ciao_rs::ciao::accounts::Account;
use ciao_rs::ciao::common::{Address, Image, Name, Timestamp};
use model::import::handlers::ClosureRecordHandler;
use model::import::{handlers::CollectingRecordHandler, Importer};
use model::{xml::config::Configuration, Initializable};

use crate::importers::accounts::{handle_account, Accounts};

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

#[test]
fn test_init() {
    let mut importer = Accounts::new();
    let config = Configuration::new();
    let result = importer.init(Some(config));
    assert!(result.is_ok());
}

#[test]
fn test_handle_account() {
    let account = Account {
        id: "id".into(),
        name: None,
        address: None,
        avatar: None,
        email: "email".into(),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert!(r.field_by_name("name.first").is_none());
        assert!(r.field_by_name("name.middle").is_none());
        assert!(r.field_by_name("name.last").is_none());
        assert!(r.field_by_name("address.city").is_none());
        assert!(r.field_by_name("address.postal_code").is_none());
        assert!(r.field_by_name("address.address_line_1").is_none());
        assert!(r.field_by_name("address.address_line_2").is_none());
        assert!(r.field_by_name("address.region_code").is_none());
        assert!(r.field_by_name("address.state").is_none());
        assert!(r.field_by_name("avatar.id").is_none());
        assert!(r.field_by_name("avatar.updatedAt.timeUtc").is_none());
        assert!(r.field_by_name("avatar.updatedAt.timeZone").is_none());

        assert_eq!(r.field_by_name("id").unwrap().value().to_string(), "id");
        assert_eq!(
            r.field_by_name("email").unwrap().value().to_string(),
            "email"
        );
    });
    let result = handle_account(&account, &mut handler);
    assert!(result.is_ok());
}

#[test]
fn test_handle_account_with_name() {
    let account = Account {
        id: "id".into(),
        name: Some(Name {
            first: "first".into(),
            middle: "middle".into(),
            last: "last".into(),
        }),
        address: None,
        avatar: None,
        email: "email".into(),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert!(r
            .field_by_name("name.first")
            .is_some_and(|v| v.value().to_string() == "first"));
        assert!(r
            .field_by_name("name.middle")
            .is_some_and(|v| v.value().to_string() == "middle"));
        assert!(r
            .field_by_name("name.last")
            .is_some_and(|v| v.value().to_string() == "last"));
        assert!(r.field_by_name("address.city").is_none());
        assert!(r.field_by_name("address.postal_code").is_none());
        assert!(r.field_by_name("address.address_line_1").is_none());
        assert!(r.field_by_name("address.address_line_2").is_none());
        assert!(r.field_by_name("address.region_code").is_none());
        assert!(r.field_by_name("address.state").is_none());
        assert!(r.field_by_name("avatar.id").is_none());
        assert!(r.field_by_name("avatar.updatedAt.timeUtc").is_none());
        assert!(r.field_by_name("avatar.updatedAt.timeZone").is_none());

        assert_eq!(r.field_by_name("id").unwrap().value().to_string(), "id");
        assert_eq!(
            r.field_by_name("email").unwrap().value().to_string(),
            "email"
        );
    });
    let result = handle_account(&account, &mut handler);
    assert!(result.is_ok());
}

#[test]
fn test_handle_account_with_name_and_address() {
    let account = Account {
        id: "id".into(),
        name: Some(Name {
            first: "first".into(),
            middle: "middle".into(),
            last: "last".into(),
        }),
        address: Some(Address {
            city: "city".into(),
            postal_code: "postal_code".into(),
            address_line_1: "address_line_1".into(),
            address_line_2: "address_line_2".into(),
            region_code: "region_code".into(),
            state: "state".into(),
        }),
        avatar: None,
        email: "email".into(),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert!(r
            .field_by_name("name.first")
            .is_some_and(|v| v.value().to_string() == "first"));
        assert!(r
            .field_by_name("name.middle")
            .is_some_and(|v| v.value().to_string() == "middle"));
        assert!(r
            .field_by_name("name.last")
            .is_some_and(|v| v.value().to_string() == "last"));
        assert!(r
            .field_by_name("address.city")
            .is_some_and(|v| v.value().to_string() == "city"));
        assert!(r
            .field_by_name("address.postal_code")
            .is_some_and(|v| v.value().to_string() == "postal_code"));
        assert!(r
            .field_by_name("address.address_line_1")
            .is_some_and(|v| v.value().to_string() == "address_line_1"));
        assert!(r
            .field_by_name("address.address_line_2")
            .is_some_and(|v| v.value().to_string() == "address_line_2"));
        assert!(r
            .field_by_name("address.region_code")
            .is_some_and(|v| v.value().to_string() == "region_code"));
        assert!(r
            .field_by_name("address.state")
            .is_some_and(|v| v.value().to_string() == "state"));
        assert!(r.field_by_name("avatar.id").is_none());
        assert!(r.field_by_name("avatar.updatedAt.timeUtc").is_none());
        assert!(r.field_by_name("avatar.updatedAt.timeZone").is_none());

        assert_eq!(r.field_by_name("id").unwrap().value().to_string(), "id");
        assert_eq!(
            r.field_by_name("email").unwrap().value().to_string(),
            "email"
        );
    });
    let result = handle_account(&account, &mut handler);
    assert!(result.is_ok());
}

#[test]
fn test_handle_account_with_name_and_address_and_avatar() {
    let account = Account {
        id: "id".into(),
        name: Some(Name {
            first: "first".into(),
            middle: "middle".into(),
            last: "last".into(),
        }),
        address: Some(Address {
            city: "city".into(),
            postal_code: "postal_code".into(),
            address_line_1: "address_line_1".into(),
            address_line_2: "address_line_2".into(),
            region_code: "region_code".into(),
            state: "state".into(),
        }),
        avatar: Some(Image {
            id: "id".into(),
            updated_at: Some(Timestamp {
                time_utc: Some(prost_types::Timestamp {
                    seconds: 0,
                    nanos: 0,
                }),
                time_zone: "Utc".into(),
            }),
        }),
        email: "email".into(),
    };
    let mut handler = ClosureRecordHandler::new(|r| {
        assert!(r
            .field_by_name("name.first")
            .is_some_and(|v| v.value().to_string() == "first"));
        assert!(r
            .field_by_name("name.middle")
            .is_some_and(|v| v.value().to_string() == "middle"));
        assert!(r
            .field_by_name("name.last")
            .is_some_and(|v| v.value().to_string() == "last"));
        assert!(r
            .field_by_name("address.city")
            .is_some_and(|v| v.value().to_string() == "city"));
        assert!(r
            .field_by_name("address.postal_code")
            .is_some_and(|v| v.value().to_string() == "postal_code"));
        assert!(r
            .field_by_name("address.address_line_1")
            .is_some_and(|v| v.value().to_string() == "address_line_1"));
        assert!(r
            .field_by_name("address.address_line_2")
            .is_some_and(|v| v.value().to_string() == "address_line_2"));
        assert!(r
            .field_by_name("address.region_code")
            .is_some_and(|v| v.value().to_string() == "region_code"));
        assert!(r
            .field_by_name("address.state")
            .is_some_and(|v| v.value().to_string() == "state"));
        assert!(r
            .field_by_name("avatar.id")
            .is_some_and(|v| v.value().to_string() == "id"));
        assert!(r
            .field_by_name("avatar.updatedAt.timeUtc")
            .is_some_and(|v| {
                match v.value() {
                    model::value::Value::I64(v) => v == 0,
                    _ => false
                }
            }));
        assert!(r
            .field_by_name("avatar.updatedAt.timeZone")
            .is_some_and(|v| v.value().to_string() == "Utc"));

        assert_eq!(r.field_by_name("id").unwrap().value().to_string(), "id");
        assert_eq!(
            r.field_by_name("email").unwrap().value().to_string(),
            "email"
        );
    });
    let result = handle_account(&account, &mut handler);
    assert!(result.is_ok());
}
