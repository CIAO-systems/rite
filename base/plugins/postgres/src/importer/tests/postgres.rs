use std::error::Error;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use model::{
    Initializable,
    import::{Importer, handlers::ClosureRecordHandler},
    value::Value,
    xml,
};
use postgres::Client;
use rust_decimal::{Decimal, dec};

use crate::{
    common::Connection,
    embedded::Embedded,
    importer::{PostgresImporter, config::RitePostgresImport, handle_row},
};

#[test]
fn test_handle_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut embeded = Embedded::new("test")?;
    create_dummy_table(&mut embeded)?;

    test_supported(&mut embeded.client)?;

    Ok(())
}

const CREATE_QUERY: &str = r#"
    CREATE TABLE 
        dummy(
            f1 int4, 
            f2 int8, 
            f3 text, 
            f4 bool, 
            f5 float4, 
            f6 float8, 
            f7 smallserial,
            f8 int2,
            f9 integer,
            f10 bigint,
            f11 real,
            f12 double precision,
            f13 numeric,
            f14 bytea,
            f15 bpchar,
            f16 boolean,
            f17 timestamp,
            f18 date,
            f19 time,
            f20 integer[],
            f21 smallint[],
            f22 int8[],

            f100 serial,
            f101 bigserial
        );
    "#;

const INSERT_QUERY: &str = r#"
    INSERT INTO dummy 
            (f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13,f14,f15,f16,f17,f18,f19,f20,f21,f22) 
        VALUES 
            ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22) 
        RETURNING 
            f100,f101,f1,f2,f3,f4,f5,f6,f7,f8,f9,f10,f11,f12,f13,f14,f15,f16,f17,f18,f19,f20,f21,f22
    "#;

const SELECT_QUERY: &str = "SELECT * FROM dummy";

const TEST_TS: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 10, 6)
    .unwrap()
    .and_hms_opt(8, 0, 0)
    .unwrap();

fn smallint_array() -> Vec<i16> {
    vec![1, 2, 3]
}

fn integer_array() -> Vec<i32> {
    vec![1, 2, 3]
}

fn bigint_array() -> Vec<i64> {
    vec![1, 2, 3]
}

fn create_dummy_table(embeded: &mut Embedded) -> Result<(), Box<dyn Error>> {
    let mut transaction = embeded.client.transaction()?;

    transaction.execute(CREATE_QUERY, &[])?;
    let blob: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let params = (
        73,                                            // f1
        73 as i64,                                     // f2
        "This are not the droids you are looking for", // f3
        true,                                          // f4
        73.0 as f32,                                   // f5
        73.0 as f64,                                   // f6
        73 as i16,                                     // f7
        42 as i16,                                     // f8
        4273 as i32,                                   // f9
        4273_7342 as i64,                              // f10
        4273.7342 as f32,                              // f11
        4273.7342 as f64,                              // f12
        dec!(42.73),                                   // f13
        blob,                                          // f14
        "Another text",                                // f15
        false,                                         // f16
        TEST_TS,                                       // f17
        TEST_TS.date(),                                // f18
        TEST_TS.time(),                                // f19
        integer_array(),                               // f20
        smallint_array(),                              // f21
        bigint_array(),                                // f22
    );
    let rec = transaction.query_one(
        INSERT_QUERY,
        &[
            &params.0, &params.1, &params.2, &params.3, &params.4, &params.5, &params.6, &params.7,
            &params.8, &params.9, &params.10, &params.11, &params.12, &params.13, &params.14,
            &params.15, &params.16, &params.17, &params.18, &params.19, &params.20, &params.21,
        ],
    )?;
    test_insert(rec);

    transaction.commit()?;

    Ok(())
}

fn test_insert(rec: postgres::Row) {
    let value: i32 = rec.get("f100");
    assert_eq!(value, 1);
    let value: i64 = rec.get("f101");
    assert_eq!(value, 1);
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
    let value: i16 = rec.get("f8");
    assert_eq!(value, 42);
    let value: i32 = rec.get("f9");
    assert_eq!(value, 4273);
    let value: i64 = rec.get("f10");
    assert_eq!(value, 4273_7342);
    let value: f32 = rec.get("f11");
    assert_eq!(value, 4273.7342);
    let value: f64 = rec.get("f12");
    assert_eq!(value, 4273.7342);
    let value: Decimal = rec.get("f13");
    assert_eq!(value, dec!(42.73));
    let value: Vec<u8> = rec.get("f14");
    assert_eq!(value, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let value: &str = rec.get("f15");
    assert_eq!(value, "Another text");
    let value: bool = rec.get("f16");
    assert_eq!(value, false);
    let value: NaiveDateTime = rec.get("f17");
    assert_eq!(value, TEST_TS);
    let value: NaiveDate = rec.get("f18");
    assert_eq!(value, TEST_TS.date());
    let value: NaiveTime = rec.get("f19");
    assert_eq!(value, TEST_TS.time());
    let value: Vec<i16> = rec.get("f21");
    assert_eq!(value, vec![1, 2, 3]);
    let value: Vec<i32> = rec.get("f20");
    assert_eq!(value, vec![1, 2, 3]);
    let value: Vec<i64> = rec.get("f22");
    assert_eq!(value, vec![1, 2, 3]);
}

fn test_supported(client: &mut Client) -> Result<model::record::Record, Box<dyn Error>> {
    let recs = client.query(SELECT_QUERY, &[])?;
    assert_eq!(recs.len(), 1);
    let row = recs.first();
    assert!(row.is_some());
    let row = row.unwrap();

    let record = handle_row(row.clone())?;

    let f = record.field_by_name("f100");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I32(1)));

    let f = record.field_by_name("f101");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I64(1)));

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

    let f = record.field_by_name("f7");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I16(73)));

    let f = record.field_by_name("f8");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I16(42)));

    let f = record.field_by_name("f9");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I32(4273)));

    let f = record.field_by_name("f10");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::I64(4273_7342)));

    let f = record.field_by_name("f11");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::F32(4273.7342)));

    let f = record.field_by_name("f12");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::F64(4273.7342)));

    let f = record.field_by_name("f13");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Decimal(_)));
    if let Value::Decimal(d) = value {
        assert_eq!(dec!(42.73), d);
    }

    let f = record.field_by_name("f14");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Blob(_)));

    let f = record.field_by_name("f15");
    let value = f.unwrap().value();
    assert!(matches!(value, Value::String(_)));
    assert_eq!(value.to_string(), "Another text");

    let f = record.field_by_name("f16");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Bool(false)));

    let f = record.field_by_name("f17");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::DateTime(_)));
    if let Value::DateTime(ts) = value {
        assert_eq!(ts, TEST_TS);
    }

    let f = record.field_by_name("f18");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Date(_)));
    if let Value::Date(date) = value {
        assert_eq!(date, TEST_TS.date());
    }

    let f = record.field_by_name("f19");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Time(_)));
    if let Value::Time(time) = value {
        assert_eq!(time, TEST_TS.time());
    }

    let f = record.field_by_name("f20");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Collection(_)));
    if let Value::Collection(collection) = value {
        for i in 0..=2i32 {
            if let Value::I32(v) = collection[i as usize] {
                assert_eq!(i + 1, v);
            }
        }
    }

    let f = record.field_by_name("f21");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Collection(_)));
    if let Value::Collection(collection) = value {
        for i in 0..=2i16 {
            if let Value::I16(v) = collection[i as usize] {
                assert_eq!(i + 1, v);
            }
        }
    }

    let f = record.field_by_name("f22");
    assert!(f.is_some());
    let value = f.unwrap().value();
    assert!(matches!(value, Value::Collection(_)));
    if let Value::Collection(collection) = value {
        for i in 0..=2i64 {
            if let Value::I64(v) = collection[i as usize] {
                assert_eq!(i + 1, v);
            }
        }
    }

    Ok(record)
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
