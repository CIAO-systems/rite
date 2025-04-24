use std::fs;

use import::{handlers::CollectingRecordHandler, Importer};
use mockito::Matcher;
use model::{
    field::Field, record::Record, value::Value, xml::config::Configuration, Initializable,
};
use uuid::Uuid;

use crate::importer::{
    split, CONFIG_AUTH_APIKEY, CONFIG_AUTH_BASIC, CONFIG_AUTH_BEARER, HEADER_X_API_KEY,
};

use super::{record_from_json, RESTImporter, CONFIG_FIELDS_PATH, CONFIG_RECORDS_FIELD, CONFIG_URL};

static EXPECTED: [(&'static str, &'static str); 6] = [
    ("4", "A New Hope"),
    ("5", "The Empire Strikes Back"),
    ("6", "Return of the Jedi"),
    ("1", "The Phantom Menace"),
    ("2", "Attack of the Clones"),
    ("3", "Revenge of the Sith"),
];

#[test]
fn test_record_from_json() {
    // Create a sample JSON object to test
    let sample_json: serde_json::Value = serde_json::from_str(
        r#"
        {
            "name": "Tatooine",
            "climate": "arid",
            "population": "200000",
            "features": ["desert", "canyons"],
            "details": {
                "rotation_period": 23,
                "orbital_period": 304
            }
        }
        "#,
    )
    .unwrap();

    let record = record_from_json(&sample_json, &None);
    println!("{:?}", record);

    let field_option = record.field_by_name("name");
    assert!(field_option.is_some());
    if let Some(field) = field_option {
        assert_eq!("name", field.name());
        if let Value::String(value) = field.value() {
            assert_eq!("Tatooine", value);
        }
    }

    let field_option = record.field_by_name("climate");
    assert!(field_option.is_some());
    if let Some(field) = field_option {
        assert_eq!("climate", field.name());
        if let Value::String(value) = field.value() {
            assert_eq!("arid", value);
        }
    }

    let field_option = record.field_by_name("population");
    assert!(field_option.is_some());
    if let Some(field) = field_option {
        assert_eq!("population", field.name());
        if let Value::String(value) = field.value() {
            assert_eq!("200000", value);
        }
    }
}

#[test]
fn test_records_in_results() -> Result<(), Box<dyn std::error::Error>> {
    let json: serde_json::Value = serde_json::from_str(&fs::read_to_string(
        "../../../data/test/records-in-results.json",
    )?)?;
    let results = &json["results"];

    assert_results(EXPECTED, results);

    Ok(())
}

#[test]
fn test_records_in_root() -> Result<(), Box<dyn std::error::Error>> {
    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("../../../data/test/records-in-root.json")?)?;

    assert_results(EXPECTED, &json);

    Ok(())
}

// List of different SWAPI providers
static SWAPI: [(&str, Option<&str>, Option<&str>); 3] = [
    ("https://swapi.dev", Some("results"), None),
    ("https://swapi.info", None, None),
    ("https://swapi.tech", Some("result"), Some("properties")),
];

fn create_importer(swapi_index: usize) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
    let mut importer = RESTImporter::new();
    let mut config = Configuration::new();
    config.insert(
        CONFIG_URL.to_string(),
        format!("{}/api/films", SWAPI[swapi_index].0).to_string(),
    );
    if let Some(results_field) = SWAPI[swapi_index].1 {
        config.insert(CONFIG_RECORDS_FIELD.to_string(), results_field.to_string());
    }
    if let Some(fields_path) = SWAPI[swapi_index].2 {
        config.insert(CONFIG_FIELDS_PATH.to_string(), fields_path.to_string());
    }

    importer.init(Some(config))?;

    Ok(Box::new(importer))
}

#[test]
fn test_importer_in_results() -> Result<(), Box<dyn std::error::Error>> {
    for index in 1..=2 {
        println!("Testing SWAPI {}: {}...", index, SWAPI[index].0);
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        let mut importer = create_importer(index)?;
        importer.read(&mut handler)?;

        assert_result_records(EXPECTED, &records);
    }

    Ok(())
}

fn assert_result_records(expected: [(&str, &str); 6], results: &Vec<Record>) {
    assert_eq!(6, results.len());
    let mut index = 0;
    for record in results {
        let episode = record
            .field_by_name("episode_id")
            .unwrap_or(&Field::default())
            .value();
        let title = record
            .field_by_name("title")
            .unwrap_or(&Field::default())
            .value();
        let values = expected[index];
        assert_eq!(values.0, episode.to_string());
        assert_eq!(values.1, title.to_string());

        println!("Episode {} = {}", episode, title,);
        index += 1;
    }
}

fn assert_results(expected: [(&str, &str); 6], results: &serde_json::Value) {
    assert!(results.is_array());
    if let serde_json::Value::Array(array) = results {
        assert_eq!(6, array.len());
        let records: Vec<Record> = array
            .iter()
            .map(|json_record| record_from_json(json_record, &None))
            .collect();

        assert_result_records(expected, &records);
    }
}

#[test]
fn test_setup_authentication_basic() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let mut importer = RESTImporter::new();
    let mut config = Configuration::new();
    config.insert_str(CONFIG_AUTH_BASIC, "user:password");
    importer.init(Some(config))?;

    let mut server = mockito::Server::new();
    let url = server.url();
    let mock = server
        .mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .match_header(
            "Authorization",
            Matcher::Exact("Basic dXNlcjpwYXNzd29yZA==".to_string()),
        )
        .with_body("{}")
        .create();

    let client = reqwest::blocking::Client::new();
    let request = importer.setup_authentication(client.get(url));
    let response = request.send()?;
    mock.assert();
    assert_eq!(response.status(), 200);

    Ok(())
}

#[test]
fn test_setup_authentication_bearer() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let expected_token = Uuid::new_v4();
    let mut importer = RESTImporter::new();
    let mut config = Configuration::new();
    config.insert_str(CONFIG_AUTH_BEARER, &expected_token.to_string());
    importer.init(Some(config))?;

    let mut server = mockito::Server::new();
    let url = server.url();
    let mock = server
        .mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .match_header(
            "Authorization",
            Matcher::Exact(format!("Bearer {expected_token}").to_string()),
        )
        .with_body("{}")
        .create();

    let client = reqwest::blocking::Client::new();
    let request = importer.setup_authentication(client.get(url));
    let response = request.send()?;
    mock.assert();
    assert_eq!(response.status(), 200);

    Ok(())
}

#[test]
fn test_setup_authentication_apikey_custom() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let expected_token = Uuid::new_v4();
    let mut importer = RESTImporter::new();
    let mut config = Configuration::new();
    config.insert_str(
        CONFIG_AUTH_APIKEY,
        &format!("x-custom-header:{}", expected_token.to_string()),
    );
    importer.init(Some(config))?;

    let mut server = mockito::Server::new();
    let url = server.url();
    let mock = server
        .mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .match_header(
            "x-custom-header",
            Matcher::Exact(expected_token.to_string()),
        )
        .with_body("{}")
        .create();

    let client = reqwest::blocking::Client::new();
    let request = importer.setup_authentication(client.get(url));
    let response = request.send()?;
    mock.assert();
    assert_eq!(response.status(), 200);

    Ok(())
}

#[test]
fn test_setup_authentication_apikey_default() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let expected_token = Uuid::new_v4();
    let mut importer = RESTImporter::new();
    let mut config = Configuration::new();
    config.insert_str(
        CONFIG_AUTH_APIKEY,
        &format!(":{}", expected_token.to_string()),
    );
    importer.init(Some(config))?;

    let mut server = mockito::Server::new();
    let url = server.url();
    let mock = server
        .mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .match_header(HEADER_X_API_KEY, Matcher::Exact(expected_token.to_string()))
        .with_body("{}")
        .create();

    let client = reqwest::blocking::Client::new();
    let request = importer.setup_authentication(client.get(url));
    let response = request.send()?;
    mock.assert();
    assert_eq!(response.status(), 200);

    Ok(())
}

#[test]
fn test_split_basic() {
    let input = "hello:world";
    let result = split(input);
    assert_eq!(
        result,
        (Some("hello".to_string()), Some("world".to_string()))
    );
}

#[test]
fn test_split_no_colon() {
    let input = "helloworld";
    let result = split(input);
    assert_eq!(result, (Some("helloworld".to_string()), None));
}

#[test]
fn test_split_multiple_colons() {
    let input = "hello:world:rust";
    let result = split(input);
    assert_eq!(
        result,
        (Some("hello".to_string()), Some("world:rust".to_string()))
    );
}

#[test]
fn test_split_empty_string() {
    let input = "";
    let result = split(input);
    assert_eq!(result, (None, None));
}

#[test]
fn test_split_leading_colon() {
    let input = ":world";
    let result = split(input);
    assert_eq!(result, (None, Some("world".to_string())));
}

#[test]
fn test_split_trailing_colon() {
    let input = "hello:";
    let result = split(input);
    assert_eq!(result, (Some("hello".to_string()), None));
}
