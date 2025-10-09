use serde::{Deserialize, Serialize};

use model::xml::common::DatabaseConnection;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-postgres-export")]
pub struct RitePostgresExport {
    pub connection: DatabaseConnection,
    pub table: Table,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@uniqueFields")]
    pub unique_fields: Option<String>,
    pub create: Option<String>,
}

impl Table {
    pub fn get_unique_fields_as_vec(&self) -> Vec<String> {
        if let Some(ref input) = self.unique_fields {
            return input.split(',').map(|s| s.trim().to_string()).collect();
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::exporter::config::RitePostgresExport;

    use super::Table;
    #[test]
    fn test_unique_fields_ser() {
        let table = Table {
            name: "Name".to_string(),
            create: Some("CREATE-Statement".to_string()),
            unique_fields: Some("field1,field2".to_string()),
        };

        let x = serde_xml_rs::to_string(&table);
        assert!(x.is_ok());
        if let Ok(x) = x {
            assert_eq!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Table name=\"Name\" uniqueFields=\"field1,field2\"><create>CREATE-Statement</create></Table>".to_string(),
            x
        );
            println!("{:?}", x);
        }
    }

    #[test]
    fn test_unique_fields_de() {
        let xml = r#"<Table name="Name" uniqueFields="field1,field2">
            <create>CREATE-Statement</create>
        </Table>"#;

        let x: Table = serde_xml_rs::from_str(xml).unwrap();
        assert_eq!("Name", x.name);
        println!("{:?}", x);
    }

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

    #[test]
    fn test_unique_fields_as_vec_empty() {
        let xml = r#"<table name="Name"/>"#;

        let table: Table = serde_xml_rs::from_str(xml).unwrap();
        // println!("{:?}", table);
        let v = table.get_unique_fields_as_vec();
        assert!(v.is_empty());
    }

    #[test]
    fn test_unique_fields_as_vec_non_empty() {
        let xml = r#"<table name="Name" uniqueFields="a,b,c"/>"#;

        let table: Table = serde_xml_rs::from_str(xml).unwrap();
        // println!("{:?}", table);
        let v = table.get_unique_fields_as_vec();
        assert_eq!(v.len(), 3);
        assert_eq!(v.get(0).unwrap(), "a");
        assert_eq!(v.get(1).unwrap(), "b");
        assert_eq!(v.get(2).unwrap(), "c");
    }
}
