use crate::exporter::config::RitePostgresExport;
use model::xml::file::load_and_substitute_from_env;
use std::collections::HashMap;

static EXAMPLE_XML: &str = "../../data/test/test-postgres-export-config.xml";

#[test]
fn test_generate_create_statement_from_xml() -> Result<(), Box<dyn std::error::Error>> {
    let xml_contents = load_and_substitute_from_env(EXAMPLE_XML, &HashMap::new())?;

    let config: RitePostgresExport = match serde_xml_rs::from_str(&xml_contents) {
        Ok(c) => c,
        Err(e) => return Err(format!("Cannot parse contents from {}: {}", EXAMPLE_XML, e).into()),
    };

    assert!(config.table.create.is_some());
    if let Some(create) = config.table.create {
        assert_eq!(
            "CREATE TABLE backup_customer (\n\tid serial4 NOT NULL,\n\t\"name\" varchar NOT NULL,\n\tCONSTRAINT backup_customers_pkey PRIMARY KEY (id)\n);",
            create
        );
    }

    Ok(())
}
