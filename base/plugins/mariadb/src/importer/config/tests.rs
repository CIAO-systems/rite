use model::xml::file::load_and_substitute_from_env;

use crate::importer::config::RiteMariaDBImport;

#[test]
fn test() {
    // Example usage
    let xml = r#"
        <rite-mariadb-import>
            <connection 
                host="localhost"
                port="3306"
                database="mariadb"
                user="user"
                password="${MARIADB_PASSWORD:topsecret}"
            />
            <sql>select * from customers</sql>
        </rite-mariadb-import>
        "#;

    let config: RiteMariaDBImport = serde_xml_rs::from_str(xml).unwrap();
    println!("{:#?}", config);

    assert_eq!("localhost", config.connection.host);
    assert_eq!(3306, config.connection.port);
    assert_eq!("mariadb", config.connection.database);
    assert_eq!("user", config.connection.user);
    assert_eq!("${MARIADB_PASSWORD:topsecret}", config.connection.password);
    assert_eq!("select * from customers", config.sql);
}

#[test]
fn test_file() -> Result<(), Box<dyn std::error::Error>> {
    let xml_file = "../../data/test/mariadb/mariadb-import-config.xml";
    match load_and_substitute_from_env(xml_file, &std::collections::HashMap::new()) {
        Ok(xml_contents) => {
            let config: RiteMariaDBImport = match serde_xml_rs::from_str(&xml_contents) {
                Ok(x) => x,
                Err(e) => {
                    return Err(format!("Cannot parse contents from {}: {}", xml_file, e).into());
                }
            };

            assert_eq!("localhost", config.connection.host);
            assert_eq!(3306, config.connection.port);
            assert_eq!("mariadb", config.connection.database);
            assert_eq!("user", config.connection.user);
            assert_eq!("topsecret", config.connection.password);
            assert_eq!("select * from customers", config.sql);
        }
        Err(e) => panic!("{}", e),
    }

    Ok(())
}
