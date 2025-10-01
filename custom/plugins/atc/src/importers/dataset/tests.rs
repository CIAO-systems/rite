use model::{
    import::{handlers::ClosureRecordHandler, Importer},
    xml::config::Configuration,
    Initializable,
};

use crate::{
    connection::{
        clients::manager::tests::mocks::get_mock_client_manager,
        config::CFG_FILTER_TABLE,
    },
    importers::dataset::{call_dataset_get, Dataset},
};

#[test]
fn test_importer() {
    let mut dataset = Dataset::new();
    let config = Configuration::new();
    let result = dataset.init(Some(config));
    assert!(result.is_ok());
    assert!(dataset.config.is_some());

    let mut handler = ClosureRecordHandler::new(|r| println!("{:?}", r));
    let result = dataset.read(&mut handler);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "url not configured"));
}

#[tokio::test]
async fn test_importer_with_mock_server() {
    let cm = get_mock_client_manager(50053).await;
    assert!(cm.is_ok());

    let mut config = Configuration::new();
    config.insert_str(CFG_FILTER_TABLE, "table");

    let mut expected_record_found = false;
    let mut handler = ClosureRecordHandler::new(|r| {
        expected_record_found = r
            .field_by_name("table")
            .is_some_and(|f| f.value().to_string() == "table");
        println!("{:?}", r);
    });

    let result = call_dataset_get(&config, cm.unwrap().dataset_client, &mut handler).await;
    println!("{:?}", result);

    assert!(expected_record_found);
}

#[tokio::test]
async fn test_call_dataset_get_err() {
    let cm = get_mock_client_manager(50054).await;
    assert!(cm.is_ok());

    let config = Configuration::new();
    let mut handler = ClosureRecordHandler::new(|r| {
        println!("{:?}", r);
    });
    let result = call_dataset_get(&config, cm.unwrap().dataset_client, &mut handler).await;
    println!("{:?}", result);

    assert!(result.is_err_and(|e| e.to_string() == "Parameter 'filter.table' missing"));
}
