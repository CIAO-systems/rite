use std::collections::HashMap;

use model::{Initializable, export::Exporter, xml::file::load_and_substitute_from_env};
use rusqlite::Connection;

use crate::exporter::config::RiteSQLiteExport;

mod config;

pub struct SQLiteExporter {
    connection: Option<Connection>,
    created: bool,
}

impl SQLiteExporter {
    pub(crate) fn new() -> Self {
        SQLiteExporter {
            connection: None,
            created: false,
        }
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
                    && let Some(create) = sqlite_config.table.create
                {
                    // Create table
                    connection.execute(&create, [])?;
                    self.created = true;
                }

                self.connection = Some(connection);
            }
        }
        Ok(())
    }
}

impl Exporter for SQLiteExporter {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(connection) = &self.connection {
            insert_or_update(connection, record)?;
        }
        Ok(())
    }
}

fn insert_or_update(
    connection: &Connection,
    record: &model::record::Record,
) -> Result<(), model::BoxedError> {
    match insert(connection, record) {
        Ok(_affected) => Ok(()),
        Err(e) => {
            // when the error is a key violation, we try to update
            println!("{e}");
            Err(e)
        }
    }
}

fn insert(
    connection: &Connection,
    record: &model::record::Record,
) -> Result<usize, model::BoxedError> {
    Ok(0)
}

fn update(
    connection: &Connection,
    record: &model::record::Record,
) -> Result<usize, model::BoxedError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use model::{Initializable, xml::config::Configuration};

    use crate::exporter::SQLiteExporter;

    #[test]
    fn test_export() {
        let mut exporter = SQLiteExporter::new();
        let config = Configuration::with_xml("../../data/test/sqlite/export-config.xml");

        unsafe { std::env::set_var("RITE_CONFIG_PAT", "../../data/test/sqlite") };
        let result = exporter.init(Some(config));
        println!("{:?}", result);
    }
}
