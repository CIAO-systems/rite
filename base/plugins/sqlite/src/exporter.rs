use std::collections::HashMap;

use model::{
    Initializable,
    export::Exporter,
    xml::{file::load_and_substitute_from_env},
};
use rusqlite::Connection;

use crate::exporter::config::RiteSQLiteExport;

mod config;

pub struct SQLiteExporter {
    connection: Option<Connection>,
}

impl SQLiteExporter {
    pub(crate) fn new() -> Self {
        SQLiteExporter { connection: None }
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

                if let Some(create) = sqlite_config.table.create {
                    // Create table
                    connection.execute(&create, [])?;
                }

                self.connection = Some(connection);
            }
        }
        Ok(())
    }
}

impl Exporter for SQLiteExporter {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(_connection) = &self.connection {
            // TODO implement me
            println!("Exporting: {:?}", record);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use model::{xml::config::Configuration, Initializable};

    use crate::exporter::SQLiteExporter;

    #[test]
    fn test_export() {
        let mut exporter = SQLiteExporter::new();
        let config = Configuration::with_xml("../../data/test/sqlite/export-config.xml");
        let result = exporter.init(Some(config));
        println!("{:?}", result);
    }
}