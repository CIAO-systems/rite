use std::collections::HashMap;

use model::{
    Initializable,
    export::Exporter,
    xml::{common::Table, file::load_and_substitute_from_env},
};
use rite_sql::generate_insert_statement;
use rusqlite::{Connection, params_from_iter};

use crate::exporter::{config::RiteSQLiteExport, sql::SQLiteFlavor};

mod config;
mod sql;

pub struct SQLiteExporter {
    connection: Option<Connection>,
    table: Option<Table>,
    created: bool,
}

impl SQLiteExporter {
    pub(crate) fn new() -> Self {
        SQLiteExporter {
            connection: None,
            created: false,
            table: None,
        }
    }

    fn insert_or_update(
        &self,
        connection: &Connection,
        record: &model::record::Record,
    ) -> Result<(), model::BoxedError> {
        match self.insert(connection, record) {
            Ok(_affected) => Ok(()),
            Err(e) => {
                // when the error is a key violation, we try to update
                println!("{e}");
                Err(e)
            }
        }
    }

    fn insert(
        &self,
        connection: &Connection,
        record: &model::record::Record,
    ) -> Result<usize, model::BoxedError> {
        if let Some(table) = &self.table {
            let s = generate_insert_statement::<SQLiteFlavor>(&table.name, record)?;
            println!("{}", s.sql);
            println!("{:?}", s.params);

            let params = params_from_iter(s.params.iter());

            let r = connection.execute(&s.sql, params)?;
            return Ok(r);
        }

        Ok(0)
    }

    fn update(
        &self,
        _connection: &Connection,
        _record: &model::record::Record,
    ) -> Result<usize, model::BoxedError> {
        Ok(0)
    }
}

impl Initializable for SQLiteExporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            if let Some(xml_file) = config.xml {
                let xml = load_and_substitute_from_env(&xml_file, &HashMap::new())?;
                let sqlite_config: RiteSQLiteExport = serde_xml_rs::from_str(&xml)?;

                let connection = Connection::open(sqlite_config.filename)?;

                if !self.created
                    && let Some(ref create) = sqlite_config.table.create
                {
                    // Create table
                    connection.execute(&create, [])?;
                    self.created = true;
                }

                self.connection = Some(connection);
                self.table = Some(sqlite_config.table);
            }
        }
        Ok(())
    }
}

impl Exporter for SQLiteExporter {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(connection) = &self.connection {
            self.insert_or_update(connection, record)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use model::{
        Initializable, export::Exporter, field::add_field, record::Record, value::Value,
        xml::config::Configuration,
    };

    use crate::exporter::SQLiteExporter;

    #[test]
    fn test_export() {
        // Arrange
        let _ = std::fs::remove_file("../../data/test/sqlite/customers_export.db");
        let mut exporter = SQLiteExporter::new();
        let config = Configuration::with_xml("../../data/test/sqlite/export-config.xml");

        unsafe { std::env::set_var("RITE_CONFIG_PATH", "../../data/test/sqlite") };

        let mut record = Record::new();
        let fields = record.fields_as_mut();
        add_field(fields, "id", Value::I32(1));
        add_field(fields, "name", Value::String("Exported customer".into()));

        // Act
        let result = exporter.init(Some(config));
        assert!(result.is_ok());

        let result = exporter.write(&record);
        // Assert
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
