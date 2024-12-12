use super::{generate_insert_statement, generate_update_statement};
use crate::exporter::config::RitePostgresExport;
use model::{field::Field, record::Record, value::Value, xml::file::load_and_substitute_from_env};
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

    if let Ok(statement) = generate_insert_statement("tablename", &record) {
        assert_eq!(
            "INSERT INTO tablename (index, name) VALUES ($1, $2);",
            statement.sql
        );
    }
}

#[test]
fn test_generate_update_statement() {
    let expected = [
        Value::I32(0),
        Value::String("user@company".to_string()),
        Value::String("Some name".to_string()),
    ];

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    fields.push(Field::new_value("index".to_string(), expected[0].clone()));
    fields.push(Field::new_value("email".to_string(), expected[1].clone()));
    fields.push(Field::new_value("name".to_string(), expected[2].clone()));

    let unique_fields = ["index".to_string(), "email".to_string()].to_vec();
    if let Ok(statement) = generate_update_statement("tablename", &record, &unique_fields) {
        assert_eq!(
            "UPDATE tablename SET name = $3 WHERE index = $1 AND email = $2",
            statement.sql
        );

        assert_eq!(3, statement.params.len());
        for (i, value) in expected.iter().enumerate() {
            assert_eq!(*value, statement.params[i].0);
        }
    }
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
