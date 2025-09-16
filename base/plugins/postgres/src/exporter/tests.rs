use model::{
    export::Exporter, field::add_field, record::Record, value::Value, xml, Initializable,
};

use crate::{
    common::Connection,
    embedded::Embedded,
    exporter::{
        config::{RitePostgresExport, Table},
        PostgresExporter,
    },
};

#[test]
#[ignore = "not yet finished"]
fn test_export() -> Result<(), Box<dyn std::error::Error>> {
    let embeded = Embedded::new("test")?;

    let mut exporter = PostgresExporter::new();
    let config = xml::config::Configuration::with_xml("../../data/postgres-export-config.xml");
    exporter.init(Some(config))?;

    // Overwrite config
    let settings = embeded.postgresql.settings();
    exporter.postgres = Some(RitePostgresExport {
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
                "CREATE TABLE IF NOT EXISTS dummy (id serial4 NOT NULL, f1 text, f2 int4)"
                    .to_string(),
            ),
            unique_fields: Some("id".to_string()),
        },
    });

    let mut record = Record::new();
    // add_field(
    //     record.fields_as_mut(),
    //     "f1",
    //     Value::String("value".to_string()),
    // );
    add_field(
        record.fields_as_mut(),
        "f2",
        Value::I32(73),
    );

    exporter.write(&record)?;

    Ok(())
}
