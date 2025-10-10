use serde::{Deserialize, Serialize};

use model::xml::common::{DatabaseConnection, Table};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-postgres-export")]
pub struct RitePostgresExport {
    pub connection: DatabaseConnection,
    pub table: Table,
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
            <table name="backup_customer" uniqueFields="id">
                <create>CREATE TABLE IF NOT EXISTS backup_customer 
                (
	                id serial4 NOT NULL,
	                "name" varchar NOT NULL,
	                CONSTRAINT backup_customers_pkey PRIMARY KEY (id)
                )
                </create>
            </table>
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
        assert!(config.table.create.is_some());
        assert_eq!(Some("id".to_string()), config.table.unique_fields);
    }
}
