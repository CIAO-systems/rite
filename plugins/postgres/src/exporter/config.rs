use serde::{Deserialize, Serialize};

use crate::common::Connection;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-postgres-export")]
pub struct RitePostgresExport {
    pub connection: Connection,
    pub table: Table,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub create: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::exporter::config::RitePostgresExport;

    #[test]
    fn test() {
        // Example usage
        let xml = r#"
        <rite-postgres-export>
            <connection 
                host="localhost"
                port="5432"
                database="postgres"
                user="postgres"
                password="${POSTGRES_PASSWROD:topsecret}"
            />
            <table name="backup_customer" create="true"/>
        </rite-postgres-export>
        "#;

        let config: RitePostgresExport = serde_xml_rs::from_str(xml).unwrap();
        println!("{:#?}", config);

        assert_eq!("localhost", config.connection.host);
        assert_eq!(5432, config.connection.port);
        assert_eq!("postgres", config.connection.database);
        assert_eq!("postgres", config.connection.user);
        assert_eq!("${POSTGRES_PASSWROD:topsecret}", config.connection.password);
        assert_eq!("backup_customer", config.table.name);
    }
}
