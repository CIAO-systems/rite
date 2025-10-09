use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use model::field::Field;
use model::import::{Importer, RecordHandler};
use model::record::Record;
use model::value::Value;
use model::xml::file::load_and_substitute_from_env;
use model::{
    Initializable,
    xml::{self},
};
use mysql::consts::{ColumnFlags, ColumnType};
use mysql::prelude::Queryable;
use mysql::{Opts, Pool, consts};
use rust_decimal::Decimal;

use crate::importer::config::RiteMariaDBImport;
use crate::importer::tablerow::TableRow;

mod config;
mod tablerow;

pub struct MariaDBImporter {
    mariadb: Option<RiteMariaDBImport>,
}

impl MariaDBImporter {
    pub fn new() -> Self {
        MariaDBImporter { mariadb: None }
    }
}

impl Initializable for MariaDBImporter {
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let mariadb: config::RiteMariaDBImport =
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
                        self.mariadb = Some(mariadb);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
}

impl Importer for MariaDBImporter {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mariadb) = self.mariadb {
            // connect to database
            // execute query
            let connection_string = format!(
                "mysql://{user}:{password}@{host}:{port}/{database}",
                host = mariadb.connection.host,
                port = mariadb.connection.port,
                user = mariadb.connection.user,
                password = mariadb.connection.password,
                database = mariadb.connection.database
            );

            let pool = Pool::new(Opts::from_url(&connection_string)?)?;
            let mut client = pool.get_conn()?;
            // Execute the query
            let rows = client.query_iter(&mariadb.sql)?;

            // convert each row to a Record and send it to the callback
            for row in rows {
                let row = row?;

                let mut record = handle_row(&row)?;
                handler.handle_record(&mut record)?;
            }
        }

        Ok(())
    }
}

fn handle_row<R: TableRow>(row: &R) -> Result<Record, Box<dyn std::error::Error>> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    // Pre allocate memory for expected field count
    let columns = row.columns();
    fields.reserve_exact(columns.len());

    for (index, column) in columns.iter().enumerate() {
        let col_type = column.column_type();
        let col_name = &column.name_str().to_string();
        let col_flags = &column.flags();
        match col_type {
            ColumnType::MYSQL_TYPE_DECIMAL | ColumnType::MYSQL_TYPE_NEWDECIMAL => {
                handle_decimal(row, fields, index, col_name)
            }
            ColumnType::MYSQL_TYPE_TINY => handle_tiny(row, fields, index, col_name, col_flags),
            ColumnType::MYSQL_TYPE_SHORT => handle_short(row, fields, index, col_name, col_flags),
            ColumnType::MYSQL_TYPE_LONG
            | ColumnType::MYSQL_TYPE_YEAR
            | ColumnType::MYSQL_TYPE_INT24 => handle_long(row, fields, index, col_name, col_flags),
            ColumnType::MYSQL_TYPE_LONGLONG => {
                handle_longlong(row, fields, index, col_name, col_flags)
            }
            ColumnType::MYSQL_TYPE_FLOAT => handle_float(row, fields, index, col_name),
            ColumnType::MYSQL_TYPE_DOUBLE => handle_double(row, fields, index, col_name),
            ColumnType::MYSQL_TYPE_NULL => handle_null(fields, col_name),
            ColumnType::MYSQL_TYPE_VARCHAR
            | ColumnType::MYSQL_TYPE_ENUM
            | ColumnType::MYSQL_TYPE_TINY_BLOB
            | ColumnType::MYSQL_TYPE_MEDIUM_BLOB
            | ColumnType::MYSQL_TYPE_LONG_BLOB
            | ColumnType::MYSQL_TYPE_BLOB
            | ColumnType::MYSQL_TYPE_VAR_STRING
            | ColumnType::MYSQL_TYPE_SET
            | ColumnType::MYSQL_TYPE_JSON => handle_string(row, fields, index, col_name),
            ColumnType::MYSQL_TYPE_STRING | ColumnType::MYSQL_TYPE_GEOMETRY => {
                handle_blob(row, fields, index, col_name)
            }
            ColumnType::MYSQL_TYPE_TIMESTAMP
            | ColumnType::MYSQL_TYPE_DATETIME2
            | ColumnType::MYSQL_TYPE_DATETIME
            | ColumnType::MYSQL_TYPE_TIMESTAMP2 => handle_timestamp(row, fields, index, col_name),
            ColumnType::MYSQL_TYPE_DATE | ColumnType::MYSQL_TYPE_NEWDATE => {
                handle_date(row, fields, index, col_name)
            }
            ColumnType::MYSQL_TYPE_TIME | ColumnType::MYSQL_TYPE_TIME2 => {
                handle_time(row, fields, index, col_name)
            }

            // Yet unsupported types:
            // ColumnType::MYSQL_TYPE_BIT => todo!(),
            // ColumnType::MYSQL_TYPE_TYPED_ARRAY => todo!(),
            // ColumnType::MYSQL_TYPE_VECTOR => todo!(),
            // ColumnType::MYSQL_TYPE_UNKNOWN => todo!(),
            _ => return Err(format!("Unsupported type: {:?} for {}", col_type, col_name).into()),
        }
    }

    Ok(record)
}

fn handle_time<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<NaiveTime>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::Time(value)));
    });
}

fn handle_date<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<NaiveDate>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::Date(value)));
    });
}

fn handle_timestamp<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<NaiveDateTime>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::DateTime(value)));
    });
}

fn handle_blob<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<Vec<u8>>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::Blob(value)));
    });
}

fn handle_string<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<String>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::String(value)));
    });
}

fn handle_null(fields: &mut Vec<Field>, col_name: &str) {
    fields.push(Field::new_value(col_name, Value::None));
}

fn handle_double<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<f64>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::F64(value)));
    });
}

fn handle_float<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<f32>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::F32(value)));
    });
}

fn handle_longlong<R: TableRow>(row: &R,
    fields: &mut Vec<Field>,
    index: usize,
    col_name: &str,
    col_flags: &ColumnFlags,
) {
    if col_flags.contains(consts::ColumnFlags::UNSIGNED_FLAG) {
        row.get::<u64>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::U64(value)));
        });
    } else {
        row.get::<i64>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::I64(value)));
        });
    }
}

fn handle_long<R: TableRow>(row: &R,
    fields: &mut Vec<Field>,
    index: usize,
    col_name: &str,
    col_flags: &ColumnFlags,
) {
    if col_flags.contains(consts::ColumnFlags::UNSIGNED_FLAG) {
        row.get::<u32>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::U32(value)));
        });
    } else {
        row.get::<i32>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::I32(value)));
        });
    }
}

fn handle_short<R: TableRow>(row: &R,
    fields: &mut Vec<Field>,
    index: usize,
    col_name: &str,
    col_flags: &ColumnFlags,
) {
    if col_flags.contains(consts::ColumnFlags::UNSIGNED_FLAG) {
        row.get::<u16>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::U16(value)));
        });
    } else {
        row.get::<i16>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::I16(value)));
        });
    }
}

fn handle_tiny<R: TableRow>(
    row: &R,
    fields: &mut Vec<Field>,
    index: usize,
    col_name: &str,
    col_flags: &ColumnFlags,
) {
    if col_flags.contains(consts::ColumnFlags::UNSIGNED_FLAG) {
        row.get::<u8>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::U8(value)));
        });
    } else {
        row.get::<i8>(index).map(|value| {
            fields.push(Field::new_value(col_name, Value::I8(value)));
        });
    }
}

fn handle_decimal<R: TableRow>(row: &R, fields: &mut Vec<Field>, index: usize, col_name: &str) {
    row.get::<Decimal>(index).map(|value| {
        fields.push(Field::new_value(col_name, Value::Decimal(value)));
    });
}

#[cfg(test)]
mod tests;
