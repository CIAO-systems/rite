use std::error::Error;

use model::{
    import::{handlers::ClosureRecordHandler, Importer},
    value::Value,
    xml, Initializable,
};
use postgres::Client;

use crate::{
    common::Connection,
    embedded::Embedded,
    importer::{config::RitePostgresImport, handle_row, PostgresImporter},
};

#[test]
fn test_handle_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut embeded = Embedded::new("test")?;
    create_dummy_table(&mut embeded)?;

    test_supported(&mut embeded.client)?;

    test_unsuported(&mut embeded.client)?;

    Ok(())
}

fn create_dummy_table(embeded: &mut Embedded) -> Result<(), Box<dyn Error>> {
    let mut transaction = embeded.client.transaction()?;

    transaction.execute(
        "CREATE TABLE dummy(f1 int4, f2 int8, f3 text, f4 bool, f5 float4, f6 float8, f7 smallserial);",
        &[],
    )?;
    let params = (
        73,
        73 as i64,
        "This are not the droids you are looking for",
        true,
        73.0 as f32,
        73.0 as f64,
        73 as i16,
    );
    let rec = transaction.query_one("INSERT INTO dummy (f1,f2,f3,f4,f5,f6,f7) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING f1,f2,f3,f4,f5,f6,f7",
        &[&params.0,&params.1,&params.2,&params.3,&params.4,&params.5,&params.6])?;
    test_insert(rec);

    transaction.commit()?;

    Ok(())
}

fn test_insert(rec: postgres::Row) {
    let value: i32 = rec.get("f1");
    assert_eq!(value, 73);
    let value: i64 = rec.get("f2");
    assert_eq!(value, 73);
    let value: &str = rec.get("f3");
    assert_eq!(value, "This are not the droids you are looking for");
    let value: bool = rec.get("f4");
    assert_eq!(value, true);
    let value: f32 = rec.get("f5");
    assert_eq!(value, 73.0);
    let value: f64 = rec.get("f6");
    assert_eq!(value, 73.0);
    let value: i16 = rec.get("f7");
    assert_eq!(value, 73);
}

fn test_supported(client: &mut Client) -> Result<model::record::Record, Box<dyn Error>> {
    let recs = client.query("SELECT f1,f2,f3,f4,f5,f6 FROM dummy", &[])?;
    assert_eq!(recs.len(), 1);
    let row = recs.first();
    assert!(row.is_some());
    let row = row.unwrap();

    let record = handle_row(row.clone())?;

    let f = record.field_by_name("f1");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I32(73)));
    let f = record.field_by_name("f2");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I64(73)));
    let f = record.field_by_name("f3");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::String(_)));
    assert_eq!(
        value.to_string(),
        "This are not the droids you are looking for"
    );
    let f = record.field_by_name("f4");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Bool(true)));
    let f = record.field_by_name("f5");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::F32(73.0)));
    let f = record.field_by_name("f6");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::F64(73.0)));
    Ok(record)
}

fn test_unsuported(client: &mut Client) -> Result<(), Box<dyn Error>> {
    let recs = client.query("SELECT f7 FROM dummy", &[])?;
    assert_eq!(recs.len(), 1);
    let row = recs.first();
    assert!(row.is_some());

    let result = handle_row(row.unwrap().clone());
    assert!(result.is_err_and(|e| e.to_string().eq("Unsupported type: int2")));

    Ok(())
}

#[test]
fn test_import() -> Result<(), Box<dyn std::error::Error>> {
    let mut embeded = Embedded::new("test")?;
    create_dummy_table(&mut embeded)?;

    let mut importer = PostgresImporter::new();
    let config = xml::config::Configuration::with_xml("../../data/postgres-import-config.xml");
    importer.init(Some(config))?;

    // Overwrite config
    let settings = embeded.postgresql.settings();
    importer.postgres = Some(RitePostgresImport {
        connection: Connection {
            host: settings.host.clone(),
            port: settings.port,
            database: "test".to_string(),
            user: settings.username.clone(),
            password: settings.password.clone(),
        },
        sql: "select f1,f2,f3,f4,f5,f6 from dummy".to_string(),
    });

    let mut count = 0;
    let mut handler = ClosureRecordHandler::new(|_record| {
        count = count + 1;
    });

    importer.read(&mut handler)?;

    assert!(count > 0);
    Ok(())
}

#[test]
fn test_table() -> Result<(), Box<dyn std::error::Error>> {
    let mut embeded = Embedded::new("test")?;
    create_dummy_table(&mut embeded)?;

    let postgres = embeded.postgresql.settings();
    println!("{:?}", postgres);
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        postgres.host, postgres.port, postgres.username, postgres.password, "test"
    );

    let mut client = postgres::Client::connect(&connection_string, postgres::NoTls)?;
    let rows = client.query("select table_schema || '.' || table_name as tablename from information_schema.tables where table_type = 'BASE TABLE'", &[])?;
    for row in rows {
        let table: &str = row.get("tablename");
        println!("{:?}", table);
    }

    Ok(())
}
