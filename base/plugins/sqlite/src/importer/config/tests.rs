use std::collections::HashMap;

use model::xml::{config::Configuration, file::load_and_substitute_from_env};

use crate::importer::config::RiteSQLiteImport;

#[test]
fn test_xml() {
    let xml = r#"
        <rite-sqlite-import>
            <filename>/tmp/demo.db</filename>
            <sql>select * from customer</sql>
        </rite-sqlite-import>
        "#;

    let config: RiteSQLiteImport = serde_xml_rs::from_str(xml).unwrap();
    assert_eq!(config.filename, "/tmp/demo.db");
    assert_eq!(config.sql, "select * from customer");
}

#[test]
fn test_xml_file() {
    let result = load_and_substitute_from_env(
        "../../data/test/sqlite/import-config.xml",
        &HashMap::new(),
    );

    assert!(result.is_ok());

    let xml = result.unwrap();
    let config: RiteSQLiteImport = serde_xml_rs::from_str(&xml).unwrap();
    assert_eq!(config.filename, "/tmp/demo.db");
    assert_eq!(config.sql, "select * from customer");
}

#[test]
fn test_config() {
    let config = Configuration::with_xml("../../data/test/sqlite/import-config.xml");
    println!("{:?}", config);
}
