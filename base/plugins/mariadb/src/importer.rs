use model::field::Field;
use model::import::{Importer, RecordHandler};
use model::record::Record;
use model::value::Value;
use model::xml::file::load_and_substitute_from_env;
use model::{
    Initializable,
    xml::{self},
};
use mysql::consts::ColumnType;
use mysql::prelude::Queryable;
use mysql::{Opts, Pool, consts};
use rust_decimal::Decimal;

use crate::importer::config::RiteMariaDBImport;

mod config;

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

                let mut record = handle_row(row)?;
                handler.handle_record(&mut record)?;
            }
        }

        Ok(())
    }
}

fn handle_row(row: mysql::Row) -> Result<Record, Box<dyn std::error::Error>> {
    let mut record = Record::new();
    let fields = record.fields_as_mut();

    // Pre allocate memory for expected field count
    let columns = row.columns();
    fields.reserve_exact(columns.len());

    for (index, column) in columns.iter().enumerate() {
        let col_type = column.column_type();
        match col_type {
            ColumnType::MYSQL_TYPE_DECIMAL | ColumnType::MYSQL_TYPE_NEWDECIMAL => {
                row.get::<Decimal, _>(index).map(|value| {
                    fields.push(Field::new_value(&column.name_str(), Value::Decimal(value)));
                });
            }
            ColumnType::MYSQL_TYPE_TINY => {
                if column.flags().contains(consts::ColumnFlags::UNSIGNED_FLAG) {
                    row.get::<u32, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::U32(value)));
                    });
                } else {
                    row.get::<i32, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::I32(value)));
                    });
                }
            }
            ColumnType::MYSQL_TYPE_SHORT => {
                if column.flags().contains(consts::ColumnFlags::UNSIGNED_FLAG) {
                    row.get::<u16, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::U16(value)));
                    });
                } else {
                    row.get::<i16, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::I16(value)));
                    });
                }
            }
            ColumnType::MYSQL_TYPE_LONG => {
                if column.flags().contains(consts::ColumnFlags::UNSIGNED_FLAG) {
                    row.get::<u32, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::U32(value)));
                    });
                } else {
                    row.get::<i32, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::I32(value)));
                    });
                }
            }
            ColumnType::MYSQL_TYPE_LONGLONG => {
                if column.flags().contains(consts::ColumnFlags::UNSIGNED_FLAG) {
                    row.get::<u64, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::U64(value)));
                    });
                } else {
                    row.get::<i64, _>(index).map(|value| {
                        fields.push(Field::new_value(&column.name_str(), Value::I64(value)));
                    });
                }
            }

            ColumnType::MYSQL_TYPE_FLOAT => {
                row.get::<f32, _>(index).map(|value| {
                    fields.push(Field::new_value(&column.name_str(), Value::F32(value)));
                });
            }
            ColumnType::MYSQL_TYPE_DOUBLE => {
                row.get::<f64, _>(index).map(|value| {
                    fields.push(Field::new_value(&column.name_str(), Value::F64(value)));
                });
            }
            ColumnType::MYSQL_TYPE_NULL => {
                fields.push(Field::new_value(&column.name_str(), Value::None));
            }
            ColumnType::MYSQL_TYPE_VARCHAR
            | ColumnType::MYSQL_TYPE_ENUM
            | ColumnType::MYSQL_TYPE_TINY_BLOB
            | ColumnType::MYSQL_TYPE_MEDIUM_BLOB
            | ColumnType::MYSQL_TYPE_LONG_BLOB
            | ColumnType::MYSQL_TYPE_BLOB
            | ColumnType::MYSQL_TYPE_VAR_STRING
            | ColumnType::MYSQL_TYPE_SET => {
                row.get::<String, _>(index).map(|value| {
                    fields.push(Field::new_value(&column.name_str(), Value::String(value)));
                });
            }
            ColumnType::MYSQL_TYPE_STRING => {
                row.get::<Vec<u8>, _>(index).map(|value| {
                    fields.push(Field::new_value(&column.name_str(), Value::Blob(value)));
                });
            }

            // TODO finish implementing
            // ColumnType::MYSQL_TYPE_TIMESTAMP => todo!(),
            // ColumnType::MYSQL_TYPE_INT24 => todo!(),
            // ColumnType::MYSQL_TYPE_DATE => todo!(),
            // ColumnType::MYSQL_TYPE_TIME => todo!(),
            // ColumnType::MYSQL_TYPE_DATETIME => todo!(),
            // ColumnType::MYSQL_TYPE_YEAR => todo!(),
            // ColumnType::MYSQL_TYPE_NEWDATE => todo!(),
            // ColumnType::MYSQL_TYPE_BIT => todo!(),
            // ColumnType::MYSQL_TYPE_TIMESTAMP2 => todo!(),
            // ColumnType::MYSQL_TYPE_DATETIME2 => todo!(),
            // ColumnType::MYSQL_TYPE_TIME2 => todo!(),
            // ColumnType::MYSQL_TYPE_TYPED_ARRAY => todo!(),
            // ColumnType::MYSQL_TYPE_VECTOR => todo!(),
            // ColumnType::MYSQL_TYPE_UNKNOWN => todo!(),
            // ColumnType::MYSQL_TYPE_JSON => todo!(),
            // ColumnType::MYSQL_TYPE_GEOMETRY => todo!(),
            _ => return Err(format!("Unsupported type: {:?}", col_type).into()),
        }
    }

    Ok(record)
}

#[cfg(test)]
mod tests {
    use model::{
        Initializable,
        import::{Importer, handlers::ClosureRecordHandler},
        xml::config::Configuration,
    };

    use crate::importer::MariaDBImporter;

    #[test]
    #[ignore = "for manual testing"]
    fn test_import_manual() {
        let mut importer = MariaDBImporter::new();
        let xml_file = "../../data/test/mariadb/mariadb-import-config.xml";
        let config = Configuration::with_xml(xml_file);
        let result = importer.init(Some(config));
        assert!(result.is_ok());
        assert!(importer.mariadb.is_some());

        let mut handler = ClosureRecordHandler::new(|r| println!("{:?}", r));
        let result = importer.read(&mut handler);
        assert!(result.is_ok());

        let mariadb = importer.mariadb.unwrap();
        assert_eq!(mariadb.connection.host, "localhost");
        assert_eq!(mariadb.connection.port, 3306);
        assert_eq!(mariadb.connection.database, "mariadb");
        assert_eq!(mariadb.connection.user, "user");
        assert_eq!(mariadb.connection.password, "topsecret");
        assert_eq!(mariadb.sql, "select * from customers");
    }
}
