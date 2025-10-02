use config::RitePostgresImport;
use model::import::{Importer, RecordHandler};
use model::{
    Initializable,
    field::Field,
    record::Record,
    value::Value,
    xml::{self, file::load_and_substitute_from_env},
};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

mod config;

#[derive(Debug)]
pub struct PostgresImporter {
    postgres: Option<RitePostgresImport>,
}

impl PostgresImporter {
    pub(crate) fn new() -> Self {
        Self { postgres: None }
    }
}

impl Initializable for PostgresImporter {
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let postgres: config::RitePostgresImport =
                            match serde_xml_rs::from_str(&xml_contents) {
                                Ok(x) => x,
                                Err(e) => {
                                    return Err(format!(
                                        "Cannot parse contents from {}: {}",
                                        xml, e
                                    )
                                    .into());
                                }
                            };
                        self.postgres = Some(postgres);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
}

impl Importer for PostgresImporter {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref postgres) = self.postgres {
            // connect to database
            // execute query
            let connection_string = format!(
                "host={} port={} user={} password={} dbname={}",
                postgres.connection.host,
                postgres.connection.port,
                postgres.connection.user,
                postgres.connection.password,
                postgres.connection.database
            );

            let mut client = postgres::Client::connect(&connection_string, postgres::NoTls)?;
            // Execute the query
            let rows = client.query(&postgres.sql, &[])?;

            // convert each row to a Record and send it to the callback
            for row in rows {
                let mut record = handle_row(row)?;
                handler.handle_record(&mut record)?;
            }
        }

        Ok(())
    }
}

/// convert a postgres::Row to a Record
/// See PostgresSQL datatypes: https://www.postgresql.org/docs/current/datatype.html
fn handle_row(row: postgres::Row) -> Result<Record, Box<dyn std::error::Error>> {
    let mut record = Record::new();
    for (idx, column) in row.columns().iter().enumerate() {
        let field_type = column.type_().name();
        match field_type {
            "smallint" | "smallserial" | "int2" => {
                let value: i16 = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::I16(value)));
            }
            "integer" | "serial" | "int4" => {
                let value: i32 = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::I32(value)));
            }
            "int8" | "bigserial" => {
                let value: i64 = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::I64(value)));
            }
            "text" | "varchar" => {
                let value: String = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::String(value)));
            }
            "bool" => {
                let value: bool = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::Bool(value)));
            }
            "float4" => {
                let value: f32 = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::F32(value)));
            }
            "float8" => {
                let value: f64 = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::F64(value)));
            }
            "numeric" => {
                let value: Decimal = row.get(idx);
                if let Some(value) = value.to_f64() {
                    record
                        .fields_as_mut()
                        .push(Field::new_value(column.name(), Value::F64(value)));
                } else {
                    return Err(format!("Cannot convert Decimal to f64: {value}").into());
                }
            }
            "bytea" => {
                let value: Vec<u8> = row.get(idx);
                record
                    .fields_as_mut()
                    .push(Field::new_value(column.name(), Value::Blob(value)));
            }
            _ => return Err(format!("Unsupported type: {}", field_type).into()),
        }
    }
    Ok(record)
}

#[cfg(test)]
mod tests;
