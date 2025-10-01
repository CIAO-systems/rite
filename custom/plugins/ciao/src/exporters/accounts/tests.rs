use model::{field::add_field, record::Record, value::Value, xml::config::Configuration, BoxedError};

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
    assert!(avatar.is_none());
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
    assert!(avatar.is_none());

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
    assert!(avatar.is_none());
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

#[test]
fn test_init() -> Result<(), BoxedError> {
    let mut exporter = Accounts::new();
    let config = Configuration::new();
    let result = exporter.init(Some(config));
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
    Ok(())
}

#[test]
fn test_write() -> Result<(), BoxedError> {
    let mut exporter = Accounts::new();
    let record = Record::new();
    let result = exporter.write(&record);
    println!("{:?}", result);
    assert!(result.is_ok());
    Ok(())
}