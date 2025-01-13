use model::xml::config::Configuration;

use crate::importers::generic::config::{Dataset, Field, Fields};
use crate::importers::connection::{YouTrackConnection, CFG_TOKEN, CFG_URL};

use crate::importers::generic::rest::create_url_from_dataset;
use crate::YouTrackImporter;

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
