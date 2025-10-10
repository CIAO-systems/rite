use model::{Initializable, export::Exporter, xml::file::load_and_substitute_from_env};
use mysql::{PooledConn, prelude::Queryable};

use crate::{connect, exporter::config::RiteMariaDBExport};

mod config;

pub struct MariaDBExporter {
    mariadb: Option<RiteMariaDBExport>,
    client: Option<PooledConn>,
    created: bool,
}

impl MariaDBExporter {
    pub(crate) fn new() -> Self {
        Self {
            mariadb: None,
            client: None,
            created: false,
        }
    }

    fn connect(&mut self) -> Result<(), model::BoxedError> {
        // connect to database
        if let Some(mariadb) = &self.mariadb {
            self.client = Some(connect(&mariadb.connection)?)
        }

        Ok(())
    }

    /// Returns true, if the [Table] has a create statement
    fn needs_creating(&self) -> bool {
        if self.created {
            return false;
        }

        if let Some(mariadb) = &self.mariadb {
            mariadb.table.create.is_some()
        } else {
            false
        }
    }

    /// Executes the create statement of the [Table]. After creation, the create statement is removed, so
    /// it does not be executed twice
    fn create(&mut self) -> Result<(), model::BoxedError> {
        if let Some(ref mut client) = self.client {
            let query = self
                .mariadb
                .as_ref()
                .map(|m| m.table.create.as_ref().map_or("", |v| v));
            if let Some(query) = query {
                // Execute the CREATE query
                client.query_drop(query)?;

                self.created = true;
            }
        }

        Ok(())
    }

    fn insert_or_update(&self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(client) = &self.client {
            // try to insert the record
            let insert_result = self.insert(client, record);
        }

        Ok(())
    }

    fn insert(
        &self,
        client: &PooledConn,
        record: &model::record::Record,
    ) -> Result<(), model::BoxedError> {
        
        Ok(())
    }
}

impl Initializable for MariaDBExporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let mariadb: config::RiteMariaDBExport =
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

                        // Connect here already, because write is called from outside
                        self.connect()?;

                        // Create table, if necessary
                        if self.needs_creating() {
                            self.create()?;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error while loading {}: {}", xml, e);
                        return Err(e.into());
                    }
                }
            }
        }
        Ok(())
    }
}

impl Exporter for MariaDBExporter {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        // Try to insert the record
        self.insert_or_update(record)?;
        Ok(())
    }
}
