use model::xml::common::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-postgres-import")]
pub struct RitePostgresImport {
    pub connection: DatabaseConnection,
    pub sql: String,
}

#[cfg(test)]
mod tests {
    use model::helper::pwd;
    use model::xml::file::load_and_substitute_from_env;

    use crate::importer::config::RitePostgresImport;

    #[test]
    fn test() {
        // Example usage
        let xml = r#"
        <rite-postgres-import>
            <connection 
                host="localhost"
                port="5432"
                database="postgres"
                user="postgres"
                password="${POSTGRES_PASSWROD:topsecret}"
            />
            <sql>select * from customers</sql>
        </rite-postgres-import>
        "#;

        let config: RitePostgresImport = serde_xml_rs::from_str(xml).unwrap();
        println!("{:#?}", config);

        assert_eq!("localhost", config.connection.host);
        assert_eq!(5432, config.connection.port);
        assert_eq!("postgres", config.connection.database);
        assert_eq!("postgres", config.connection.user);
        assert_eq!("${POSTGRES_PASSWROD:topsecret}", config.connection.password);
        assert_eq!("select * from customers", config.sql);
    }

    #[test]
    fn test_file() -> Result<(), Box<dyn std::error::Error>> {
        pwd();
        let xml_file = "../../data/postgres-import-config.xml";
        match load_and_substitute_from_env(xml_file, &std::collections::HashMap::new()) {
            Ok(xml_contents) => {
                let postgres: RitePostgresImport = match serde_xml_rs::from_str(&xml_contents) {
                    Ok(x) => x,
                    Err(e) => {
                        return Err(format!("Cannot parse contents from {}: {}", xml_file, e).into());
                    }
                };

                assert_eq!("localhost", postgres.connection.host);
                assert_eq!(5432, postgres.connection.port);
                assert_eq!("postgres", postgres.connection.database);
                assert_eq!("postgres", postgres.connection.user);
                assert_eq!(
                    "6d598907-a775-4383-ab6f-de525c5ac0bf",
                    postgres.connection.password
                );
                assert_eq!("select * from customers", postgres.sql);
            }
            Err(e) => panic!("{}", e),
        }

        Ok(())
    }
}
