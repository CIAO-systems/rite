use std::collections::HashMap;

use model::{
    Initializable, field::add_field, import::Importer, record::Record,
    xml::file::load_and_substitute_from_env,
};
use rusqlite::Connection;

use crate::importer::config::RiteSQLiteImport;

mod config;

#[derive(Debug)]
pub struct SQLiteImporter {
    connection: Option<Connection>,
    sql: Option<String>,
}

impl SQLiteImporter {
    pub(crate) fn new() -> Self {
        SQLiteImporter {
            connection: None,
            sql: None,
        }
    }
}

impl Initializable for SQLiteImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            if let Some(xml_file) = config.xml {
                let xml = load_and_substitute_from_env(&xml_file, &HashMap::new())?;
                let sqlite_config: RiteSQLiteImport = serde_xml_rs::from_str(&xml)?;
                self.connection = Some(Connection::open(sqlite_config.filename)?);
                self.sql = Some(sqlite_config.sql);
            }
        }
        Ok(())
    }
}

impl Importer for SQLiteImporter {
    fn read(
        &mut self,
        handler: &mut dyn model::import::RecordHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(connection) = &self.connection {
            if let Some(sql) = &self.sql {
                let mut statement = connection.prepare(sql)?;
                let col_names: Vec<String> = statement
                    .column_names()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                let mut rows = statement.query([])?;

                while let Some(row) = rows.next()? {
                    let mut record = Record::new();
                    let fields = record.fields_as_mut();
                    for (col_index, col_name) in col_names.iter().enumerate() {
                        match row.get_ref(col_index)? {
                            rusqlite::types::ValueRef::Null => {
                                add_field(fields, col_name, model::value::Value::None)
                            }
                            rusqlite::types::ValueRef::Integer(i) => {
                                add_field(fields, col_name, model::value::Value::I64(i))
                            }
                            rusqlite::types::ValueRef::Real(r) => {
                                add_field(fields, col_name, model::value::Value::F64(r))
                            }
                            rusqlite::types::ValueRef::Text(items) => add_field(
                                fields,
                                col_name,
                                model::value::Value::String(
                                    String::from_utf8_lossy(items).to_string(),
                                ),
                            ),
                            rusqlite::types::ValueRef::Blob(items) => add_field(
                                fields,
                                col_name,
                                model::value::Value::Blob(items.to_vec()),
                            ),
                        }
                    }

                    // Pass record to transform/export pipeline
                    handler.handle_record(&mut record)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use model::{
        Initializable,
        import::{Importer, handlers::ClosureRecordHandler},
        xml::config::Configuration,
    };

    use crate::importer::SQLiteImporter;

    #[test]
    fn test_read() {
        let mut importer = SQLiteImporter::new();
        let config = Configuration::with_xml("../../data/test/sqlite/import-config.xml");
        // Set the RITE_CONFIG_PATH per env, so it gets replaced correctly
        unsafe { std::env::set_var("RITE_CONFIG_PATH", "../../data/test/sqlite") };
        let result = importer.init(Some(config));
        assert!(result.is_ok());

        let db_path = importer.connection.as_ref().unwrap().path().unwrap();
        assert!(db_path.ends_with("base/data/test/sqlite/customers.db"));

        assert_eq!(importer.sql.as_ref().unwrap(), "select * from customers");

        let mut handler = ClosureRecordHandler::new(|r| {
            //
            println!("{:?}", r);
        });

        let result = importer.read(&mut handler);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
