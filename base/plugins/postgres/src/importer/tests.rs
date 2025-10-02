use model::import::{handlers::ClosureRecordHandler, Importer};
use model::{xml, Initializable};

use super::PostgresImporter;

#[test]
#[ignore = "for manual testing"]
fn test_import() -> Result<(), Box<dyn std::error::Error>> {
    let mut importer = PostgresImporter::new();
    let config = xml::config::Configuration::with_xml("../../data/postgres-import-config.xml");

    importer.init(Some(config))?;

    let mut count = 0;
    let mut handler = ClosureRecordHandler::new(|_record| {
        count = count + 1;
    });
    importer.read(&mut handler)?;

    assert!(count > 0);
    Ok(())
}

#[test]
fn test_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = xml::config::Configuration::with_xml("../../data/postgres-import-config.xml");
    let mut importer = PostgresImporter::new();
    importer.init(Some(config))?;

    assert!(importer.postgres.is_some());
    let postgres = importer.postgres.unwrap();
    assert_eq!(postgres.connection.host, "localhost".to_string());
    assert_eq!(postgres.connection.port, 5432);
    assert_eq!(postgres.connection.database, "postgres".to_string());
    assert_eq!(postgres.connection.user, "postgres".to_string());
    assert_eq!(
        postgres.connection.password,
        "6d598907-a775-4383-ab6f-de525c5ac0bf".to_string()
    );

    assert_eq!(postgres.sql, "select * from customers".to_string());
    Ok(())
}

mod postgres;
