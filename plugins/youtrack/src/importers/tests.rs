use crate::importers::{CFG_TOKEN, CFG_URL};

use super::{config::Dataset, rest::create_url_from_dataset, YouTrackImporter};

#[test]
fn test_check_config() {
    let mut yti = YouTrackImporter::new();
    assert_eq!(yti.check_config(), Some(CFG_URL));
    yti.url = Some("demo-url".to_string());
    assert_eq!(yti.check_config(), Some(CFG_TOKEN));
    yti.token = Some("token".to_string());
    assert_eq!(yti.check_config(), None);
}

#[test]
fn test_create_url_from_dataset1() {
    let dataset = Dataset {
        path: String::from("Pacific Crest Trail"),
        resource: Some("Gold".to_string()),
        query: None,
        sub_resource: None,
        fields: "claim".to_string(),
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
        fields: "claim".to_string(),
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
        fields: "claim".to_string(),
    };

    assert_eq!(
        "base_url/api/Pacific Crest Trail?query=length: 73&fields=claim",
        create_url_from_dataset(&dataset, "base_url")
    );
}
