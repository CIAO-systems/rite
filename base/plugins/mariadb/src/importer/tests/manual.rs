use model::{
    Initializable,
    import::{Importer, handlers::ClosureRecordHandler},
    xml::config::Configuration,
};

use crate::importer::MariaDBImporter;

#[test]
#[ignore = "for manual testing"]
fn test_import_manual() {
    let mut importer = MariaDBImporter::new();
    let xml_file = "../../data/test/mariadb/mariadb-import-config.xml";
    let config = Configuration::with_xml(xml_file);
    let result = importer.init(Some(config));
    assert!(result.is_ok());
    assert!(importer.mariadb.is_some());

    let mut handler = ClosureRecordHandler::new(|r| println!("{:?}", r));
    let result = importer.read(&mut handler);
    assert!(result.is_ok());

    let mariadb = importer.mariadb.unwrap();
    assert_eq!(mariadb.connection.host, "localhost");
    assert_eq!(mariadb.connection.port, 3306); 
    assert_eq!(mariadb.connection.database, "mariadb");
    assert_eq!(mariadb.connection.user, "user");
    assert_eq!(mariadb.connection.password, "topsecret");
    assert_eq!(mariadb.sql, "select * from customers");
}
