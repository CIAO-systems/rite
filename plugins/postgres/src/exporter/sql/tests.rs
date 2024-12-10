use super::generate_insert_statement;
use crate::exporter::{config::RitePostgresExport, sql::_generate_create_table_statement};
use model::{field::Field, record::Record, xml::file::load_and_substitute_from_env};
use std::collections::HashMap;

#[test]
fn test_generate_insert_statement() {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    fields.push(Field::new_i32("index".to_string(), 0));
    fields.push(Field::new_string(
        "name".to_string(),
        "Some name".to_string(),
    ));

    if let Ok(statement) = generate_insert_statement(&record, "tablename") {
        assert_eq!(
            "INSERT INTO tablename (index, name) VALUES (?, ?);",
            statement.sql
        );
    }
}

#[test]
fn test_generate_create_statement() {
    let mut record = Record::new();
    record.fields_as_mut().extend([
        Field::new_i32("user_id".to_string(), 0),
        Field::new_string("username".to_string(), "User name".to_string()),
        Field::new_i32("age".to_string(), 42),
        Field::new_bool("active".to_string(), true),
    ]);

    let create_table_sql = _generate_create_table_statement(&record, "users");
    println!("{}", create_table_sql);
    assert_eq!("CREATE TABLE users (user_id INTEGER NOT NULL,username TEXT NOT NULL,age INTEGER NOT NULL,active BOOLEAN NOT NULL);", 
    create_table_sql);
}

static EXAMPLE_XML: &str = "../../data/test-postgres-export-config.xml";

#[test]
fn test_generate_create_statement_from_xml() -> Result<(), Box<dyn std::error::Error>> {
    let xml_contents = load_and_substitute_from_env(EXAMPLE_XML, &HashMap::new())?;

    let config: RitePostgresExport = match serde_xml_rs::from_str(&xml_contents) {
        Ok(c) => c,
        Err(e) => return Err(format!("Cannot parse contents from {}: {}", EXAMPLE_XML, e).into()),
    };

    assert!(config.table.create.is_some());
    if let Some(create) = config.table.create {
        assert_eq!("CREATE TABLE backup_customer (\n\tid serial4 NOT NULL,\n\t\"name\" varchar NOT NULL,\n\tCONSTRAINT backup_customers_pkey PRIMARY KEY (id)\n);", create);
    }

    Ok(())
}
