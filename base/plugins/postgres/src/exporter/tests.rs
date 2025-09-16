use std::{error::Error, io::Write};

use model::{export::Exporter, field::add_field, record::Record, value::Value, xml, Initializable};

use crate::{
    common::Connection,
    embedded::Embedded,
    exporter::{
        config::{RitePostgresExport, Table},
        PostgresExporter,
    },
};
use tempfile::NamedTempFile;

fn create_test_config(embeded: &Embedded) -> Result<NamedTempFile, Box<dyn Error>> {
    let settings = embeded.postgresql.settings();
    let postgres = Some(RitePostgresExport {
        connection: Connection {
            host: settings.host.clone(),
            port: settings.port,
            database: "test".to_string(),
            user: settings.username.clone(),
            password: settings.password.clone(),
        },
        table: Table {
            name: "dummy".to_string(),
            create: Some(
                "CREATE TABLE IF NOT EXISTS dummy (id int4 NOT NULL UNIQUE, f1 text, f2 int4)".to_string(),
            ),
            unique_fields: Some("id".to_string()),
        },
    });
    let mut file = NamedTempFile::new()?;
    let xml_content = serde_xml_rs::to_string(&postgres)?;
    file.write_all(xml_content.as_bytes())?;

    Ok(file)
}

#[test]
fn test_export() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let mut embeded = Embedded::new("test")?;
    let mut exporter = PostgresExporter::new();

    // create config (the temporary file returned must live as long as the test)
    let file = create_test_config(&embeded)?;
    let filename = file.path().to_str().unwrap();

    let config = xml::config::Configuration::with_xml(filename);
    exporter.init(Some(config))?;

    // Act (insert)
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "id", Value::I32(1));
    add_field(
        record.fields_as_mut(),
        "f1",
        Value::String("value".to_string()),
    );
    add_field(record.fields_as_mut(), "f2", Value::I32(73));

    exporter.write(&record)?;

    // Assert (insert)
    let rows = embeded.client.query("select id,f1,f2 from dummy", &[])?;
    assert_eq!(rows.len(), 1);
    let first = rows.first().unwrap();

    let id: i32 = first.get("id");
    assert_eq!(id, 1);

    let f1: &str = first.get("f1");
    assert_eq!(f1, "value");

    let f2: i32 = first.get("f2");
    assert_eq!(f2, 73);

    // Act (update)
    let mut record = Record::new();
    add_field(record.fields_as_mut(), "id", Value::I32(1));
    add_field(
        record.fields_as_mut(),
        "f1",
        Value::String("new text".to_string()),
    );
    add_field(record.fields_as_mut(), "f2", Value::I32(42));

    exporter.write(&record)?;


    // Assert (update)
    let rows = embeded.client.query("select id,f1,f2 from dummy", &[])?;
    assert_eq!(rows.len(), 1, "there should only be one record");
    let first = rows.first().unwrap();

    let id: i32 = first.get("id");
    assert_eq!(id, 1);

    let f1: &str = first.get("f1");
    assert_eq!(f1, "new text");

    let f2: i32 = first.get("f2");
    assert_eq!(f2, 42);

    Ok(())
}
