use model::Initializable;
use model::import::Importer;
use model::import::handlers::{ClosureRecordHandler, CollectingRecordHandler};
use model::value::Value;
use model::xml::config::Configuration;

use crate::importers::connection::{CFG_TOKEN, CFG_URL, YouTrackConnection};
use crate::importers::generic::config::{Dataset, Field, Fields};

use crate::YouTrackImporter;
use crate::importers::generic::rest::create_url_from_dataset;

#[test]
fn test_check_config() {
    let mut yti = YouTrackImporter::new();
    assert_eq!(
        yti.check_config(),
        Some(YouTrackConnection::all_variables().to_string())
    );
    let mut config = Configuration::new();

    config.insert(CFG_URL.to_string(), "demo-url".to_string());
    yti.set_connection(&config);
    assert_eq!(yti.check_config(), Some(CFG_TOKEN.to_string()));

    config.insert(CFG_TOKEN.to_string(), "token".to_string());
    yti.set_connection(&config);
    assert_eq!(yti.check_config(), None);
}

#[test]
fn test_create_url_from_dataset1() {
    let dataset = Dataset {
        path: String::from("Pacific Crest Trail"),
        resource: Some("Gold".to_string()),
        query: None,
        sub_resource: None,
        fields: Fields {
            fields: vec![Field {
                id: "claim".to_string(),
            }],
        },
    };

    assert_eq!(
        "base_url/api/Pacific Crest Trail/Gold?fields=claim",
        create_url_from_dataset(&dataset, "base_url")
    );
}

#[test]
fn test_create_url_from_dataset2() {
    let dataset = Dataset {
        path: String::from("Pacific Crest Trail"),
        resource: None,
        query: None,
        sub_resource: None,
        fields: Fields {
            fields: vec![Field {
                id: "claim".to_string(),
            }],
        },
    };

    assert_eq!(
        "base_url/api/Pacific Crest Trail?fields=claim",
        create_url_from_dataset(&dataset, "base_url")
    );
}

#[test]
fn test_create_url_from_dataset3() {
    let dataset = Dataset {
        path: String::from("Pacific Crest Trail"),
        resource: None,
        query: Some("length: 73".to_string()),
        sub_resource: None,
        fields: Fields {
            fields: vec![Field {
                id: "claim".to_string(),
            }],
        },
    };

    assert_eq!(
        "base_url/api/Pacific Crest Trail?query=length%3A%2073&fields=claim",
        create_url_from_dataset(&dataset, "base_url")
    );
}

fn create_test_subject(url: String) -> YouTrackImporter {
    let mut importer = YouTrackImporter::new();
    let mut config = Configuration::with_xml("../../data/youtrack/tests/user.xml");
    config.insert_str(CFG_URL, &url);
    config.insert_str(CFG_TOKEN, "token");
    let result = importer.init(Some(config));
    assert!(result.is_ok());
    importer
}

fn mock_for_success(body: &str) -> mockito::ServerGuard {
    let mut server = mockito::Server::new();
    let _mock = server
        .mock("GET", mockito::Matcher::Regex(r"^/api.*".into()))
        .with_body(body)
        .with_status(200)
        .create();
    server
}

#[test]
fn test_import_array() {
    // Arrange
    let server = mock_for_success("[{\"field\": \"value\"},{\"field\": \"value\"}]"); // array of objects
    let mut importer = create_test_subject(server.url());

    // Act
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    let result = importer.read(&mut handler);

    // Assert
    println!("{:?}", result);
    println!("{:?}", records);
    assert!(result.is_ok());
    assert_eq!(records.len(), 2);
    for i in 0..records.len() {
        let first = records.get(i).unwrap();
        let field = first.field_by_name("field").unwrap();
        assert_eq!(field.value(), Value::String("value".into()));
    }
}

#[test]
fn test_import_object() {
    let server = mock_for_success("{\"field\": \"value\"}"); // one object
    let mut importer = create_test_subject(server.url());
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);
    let result = importer.read(&mut handler);
    println!("{:?}", result);
    println!("{:?}", records);
    assert!(result.is_ok());
    assert_eq!(records.len(), 1);
    let first = records.first().unwrap();
    let field = first.field_by_name("field").unwrap();
    assert_eq!(field.value(), Value::String("value".into()));
}

#[test]
fn test_read_no_config() {
    let mut importer = YouTrackImporter::new();
    let config = Configuration::new();
    assert!(importer.init(Some(config)).is_ok());

    let mut handler = ClosureRecordHandler::new(|_r| {});
    let result = importer.read(&mut handler);
    println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string().eq("Configuration key 'url' is missing")));
}

#[test]
fn test_init_error_config() {
    let mut importer = YouTrackImporter::new();
    const XML_FILE: &str = "this-file-is_not-existing";
    let config = Configuration::with_xml(XML_FILE);
    let result = importer.init(Some(config));
    assert!(result.is_err_and(|e| e.to_string().contains(XML_FILE)));
}
