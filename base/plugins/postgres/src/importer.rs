use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
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

mod config;
mod types;

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

fn map_array<'a, T>(
    row: &'a postgres::Row,
    idx: usize,
    column_name: &str,
) -> Result<Field, Box<dyn std::error::Error>>
where
    T: postgres::types::FromSql<'a> + Into<Value> + Clone,
{
    let value: Vec<T> = row.get(idx);
    let collection = Value::Collection(value.into_iter().map(|i| i.into()).collect());
    Ok(Field::new_value(column_name, collection))
}

/// convert a postgres::Row to a Record
/// See PostgresSQL datatypes: https://www.postgresql.org/docs/current/datatype.html
fn handle_row(row: postgres::Row) -> Result<Record, Box<dyn std::error::Error>> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    // Pre allocate memory for expected field count
    fields.reserve_exact(row.columns().len());

    for (idx, column) in row.columns().iter().enumerate() {
        let field_type = column.type_().oid();
        match field_type {
            types::Int2 => {
                // "smallint" | "int2"
                let value: i16 = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::I16(value)));
            }

            types::Int4 => {
                // "integer" | "serial" | "int4"
                let value: i32 = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::I32(value)));
            }

            types::Int8 => {
                // "int8" | "bigserial"
                let value: i64 = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::I64(value)));
            }
            types::Text | types::Varchar | types::Char | types::Bpchar => {
                // "text" | "bpchar" | "varchar"
                let value: String = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::String(value)));
            }
            types::Bool => {
                //"bool" | "boolean"
                let value: bool = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::Bool(value)));
            }
            types::Float4 => {
                // "float4"
                let value: f32 = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::F32(value)));
            }
            types::Float8 => {
                // "float8"
                let value: f64 = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::F64(value)));
            }
            types::Numeric => {
                // "numeric" => {
                let value: Decimal = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::Decimal(value)));
            }
            types::Bytea => {
                // "bytea"
                let value: Vec<u8> = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::Blob(value)));
            }
            types::Date => {
                // "date"
                let value: NaiveDate = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::Date(value)));
            }
            types::Time => {
                // "time"
                let value: NaiveTime = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::Time(value)));
            }
            types::Timestamp => {
                // "timestamp"
                let value: NaiveDateTime = row.get(idx);
                fields.push(Field::new_value(column.name(), Value::DateTime(value)));
            }
            types::Int2Array => {
                // "_int2"
                fields.push(map_array::<i16>(&row, idx, column.name())?);
            }
            types::Int4Array => {
                // "_int4"
                fields.push(map_array::<i32>(&row, idx, column.name())?);
            }
            types::Int8Array => {
                // "_int8"
                fields.push(map_array::<i64>(&row, idx, column.name())?);
            }
            _ => return Err(format!("Unsupported type: {} for {}", field_type, column.name()).into()),
        }
    }
    Ok(record)
}

#[cfg(test)]
mod tests;
