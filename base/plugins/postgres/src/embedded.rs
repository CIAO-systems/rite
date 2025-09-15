use std::error::Error;

use postgres::{Client, NoTls};
use postgresql_embedded::blocking::PostgreSQL;


pub struct Embedded {
    pub postgresql: PostgreSQL,
    pub client: Client,
}

impl Embedded {
    pub fn new(database: &str) -> Result<Self, Box<dyn Error>> {
        // Needs libxml2-legacy (arch) installed to work, or libxml2 (ubuntu?)
        let mut postgresql = PostgreSQL::default();
        postgresql.setup()?;
        postgresql.start()?;

        postgresql.create_database(database)?;

        let settings = postgresql.settings();
        let client = Client::connect(
            format!(
                "host={host} port={port} user={username} password={password} dbname={database}",
                host = settings.host,
                port = settings.port,
                username = settings.username,
                password = settings.password
            )
            .as_str(),
            NoTls,
        )?;

        Ok(Self { client, postgresql })
    }
}

impl Drop for Embedded {
    fn drop(&mut self) {
        let _ = self.postgresql.stop();
    }
}
