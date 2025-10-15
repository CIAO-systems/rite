use model::export::Exporter;
use mysql::{PooledConn, prelude::Queryable};
use rite_sql::{generate_insert_statement, generate_update_statement};

use crate::{
    connect,
    exporter::{config::RiteMariaDBExport, sql::MariaDBFlavor},
};

mod config;
mod initializable;
mod sql;

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

    fn insert_or_update(
        &mut self,
        record: &model::record::Record,
    ) -> Result<(), model::BoxedError> {
        if let Some(ref mut client) = self.client {
            if let Some(ref config) = self.mariadb {
                // try to insert the record
                let insert_result = insert(config, client, record);
                return match insert_result {
                    Ok(_count) => Ok(()),
                    Err(e) => {
                        if let Some(mysql::Error::MySqlError(e)) = e.downcast_ref::<mysql::Error>()
                        {
                            if e.code == sql::error_code::ER_DUP_ENTRY {
                                return match update(config, client, record) {
                                    Ok(_count) => Ok(()),
                                    Err(e) => Err(e),
                                };
                            }
                        }
                        Err(e)
                    }
                };
            }
        }

        Ok(())
    }
}

fn insert(
    config: &RiteMariaDBExport,
    client: &mut PooledConn,
    record: &model::record::Record,
) -> Result<u64, model::BoxedError> {
    let affected_rows = if let Ok(statement) =
        generate_insert_statement::<MariaDBFlavor>(&config.table.name, record)
    {
        let params: Vec<mysql::Value> = statement.params.iter().map(|p| p.into()).collect();

        client.exec_drop(&statement.sql, params)?;
        client.affected_rows()
    } else {
        0
    };
    Ok(affected_rows)
}

fn update(
    config: &RiteMariaDBExport,
    client: &mut PooledConn,
    record: &model::record::Record,
) -> Result<u64, model::BoxedError> {
    let unique_fields = config.table.get_unique_fields_as_set();
    let affected_rows = if let Ok(statement) =
        generate_update_statement::<MariaDBFlavor>(&config.table.name, record, &unique_fields)
    {
        let params: Vec<mysql::Value> = statement.params.iter().map(|p| p.into()).collect();
        println!("sql={}\nparams={:?}", statement.sql, statement.params);
        client.exec_drop(&statement.sql, params)?;

        client.affected_rows()
    } else {
        0
    };

    Ok(affected_rows)
}

impl Exporter for MariaDBExporter {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        // Try to insert the record
        self.insert_or_update(record)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
