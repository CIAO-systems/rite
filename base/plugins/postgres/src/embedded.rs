use std::{env, error::Error};

use postgres::{Client, NoTls};
use postgresql_embedded::blocking::PostgreSQL;
use uuid::Uuid;

pub struct Embedded {
    pub postgresql: PostgreSQL,
    pub client: Client,
}

impl Embedded {
    pub fn new(database: &str) -> Result<Self, Box<dyn Error>> {
        // Needs libxml2-legacy (arch) installed to work, or libxml2 (ubuntu?)
        let mut settings = PostgreSQL::default().settings().clone();
        settings.timeout = Some(std::time::Duration::from_secs(60));
        let base = env::temp_dir()
            .join("pg_embedded")
            .join(Uuid::new_v4().to_string());
        settings.data_dir = base.join("data");
        settings.installation_dir = base.join("installation");

        let mut postgresql = PostgreSQL::new(settings);

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
