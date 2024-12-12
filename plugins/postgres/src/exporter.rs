use model::{record::Record, xml::file::load_and_substitute_from_env, Initializable};
use postgres::Client;
use sql::{generate_insert_statement, generate_update_statement};

mod config;
mod sql;

pub struct PostgresExporter {
    postgres: Option<config::RitePostgresExport>,
    client: Option<Client>,
    created: bool,
}

impl PostgresExporter {
    pub fn new() -> Self {
        Self {
            postgres: None,
            client: None,
            created: false,
        }
    }

    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref config) = self.postgres {
            // connect to database
            // execute query
            let connection_string = format!(
                "host={} port={} user={} password={} dbname={}",
                config.connection.host,
                config.connection.port,
                config.connection.user,
                config.connection.password,
                config.connection.database
            );

            self.client = Some(postgres::Client::connect(
                &connection_string,
                postgres::NoTls,
            )?);
        }

        Ok(())
    }

    fn has_create(&self) -> bool {
        if let Some(ref postgres) = self.postgres {
            if postgres.table.create.is_some() {
                // We should create the table
                return true;
            }
        }
        false
    }

    fn is_created(&self) -> bool {
        self.created
    }

    fn set_created(&mut self, value: bool) {
        self.created = value;
    }

    fn create_table(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut client) = self.client {
            if let Some(ref config) = self.postgres {
                if let Some(ref create) = config.table.create {
                    client.execute(create, &[])?;
                    self.set_created(true);
                }
            }
        }

        Ok(())
    }

    fn insert_or_update(&mut self, record: &Record) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut client) = self.client {
            if let Some(ref config) = self.postgres {
                let insert_result = insert(config, record, client);
                return match insert_result {
                    Ok(count) => {
                        log::info!("Inserted {} records", count);
                        Ok(())
                    }
                    Err(e) if e.code() == Some(&postgres::error::SqlState::UNIQUE_VIOLATION) => {
                        return match update(config, record, client) {
                            Ok(count) => {
                                log::info!("Updated {} records", count);
                                Ok(())
                            }
                            Err(e) => Err(Box::new(e)),
                        };
                    }
                    Err(e) => Err(Box::new(e)),
                };
            }
        }

        Ok(())
    }
}

fn insert(
    config: &config::RitePostgresExport,
    record: &Record,
    client: &mut Client,
) -> Result<u64, postgres::Error> {
    if let Ok(statement) = generate_insert_statement(&config.table.name, record) {
        let params = statement
            .params
            .iter()
            .map(|v| v as &(dyn postgres::types::ToSql + Sync))
            .collect::<Vec<_>>();
        client.execute(&statement.sql, &params)
    } else {
        Ok(0)
    }
}

fn update(
    config: &config::RitePostgresExport,
    record: &Record,
    client: &mut Client,
) -> Result<u64, postgres::Error> {
    let unique_fields = config.table.get_unique_fields_as_vec();
    if let Ok(statement) = generate_update_statement(&config.table.name, record, &unique_fields) {
        let params = statement
            .params
            .iter()
            .map(|v| v as &(dyn postgres::types::ToSql + Sync))
            .collect::<Vec<_>>();
        client.execute(&statement.sql, &params)
    } else {
        Ok(0)
    }
}

impl Initializable for PostgresExporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let postgres: config::RitePostgresExport =
                            match serde_xml_rs::from_str(&xml_contents) {
                                Ok(x) => x,
                                Err(e) => {
                                    return Err(format!(
                                        "Cannot parse contents from {}: {}",
                                        xml, e
                                    )
                                    .into())
                                }
                            };
                        self.postgres = Some(postgres);

                        // Connect here already, because write is called from outside
                        self.connect()?;
                    }
                    Err(e) => {
                        log::error!("Error while loading {}: {}", xml, e);
                        eprintln!("Error while loading {}: {}", xml, e);
                        return Err(e.into());
                    }
                }
            }
        }
        Ok(())
    }
}

impl export::Exporter for PostgresExporter {
    fn write(&mut self, record: &Record) -> Result<(), Box<dyn std::error::Error>> {
        // Check, if we need to create the table first
        if self.has_create() && !self.is_created() {
            // Table should be created, but has not yet been created
            self.create_table()?;
        }

        // Try to insert the record
        self.insert_or_update(record)?;

        Ok(())
    }
}
