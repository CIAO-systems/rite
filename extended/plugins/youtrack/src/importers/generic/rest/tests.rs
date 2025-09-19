use model::import::{RecordHandler, handlers::CollectingRecordHandler};

use crate::importers::generic::{
    config::{Dataset, Field, Fields, RiteYoutrackImport},
    rest::{create_url_from_dataset, make_request},
};

#[test]
fn test_create_url_from_dataset_all() {
    let dataset = Dataset {
        path: "path".into(),
        resource: Some("resource".into()),
        query: Some("query".into()),
        sub_resource: Some("sub-resource".into()),
        fields: Fields {
            fields: vec![Field { id: "id1".into() }, Field { id: "id2".into() }],
        },
    };
    let result = create_url_from_dataset(&dataset, "");
    assert_eq!(result, "/api/path/resource/sub-resource?fields=id1,id2");
}

#[test]
fn test_create_url_from_dataset_min() {
    let dataset = Dataset {
        path: "path".into(),
        resource: None,
        query: None,
        sub_resource: None,
        fields: Fields { fields: vec![] },
    };
    let result = create_url_from_dataset(&dataset, "");
    assert_eq!(result, "/api/path");
}

#[test]
fn test_create_url_from_dataset_min_plus_query() {
    let dataset = Dataset {
        path: "path".into(),
        resource: None,
        query: Some("query".into()),
        sub_resource: None,
        fields: Fields { fields: vec![] },
    };
    let result = create_url_from_dataset(&dataset, "");
    assert_eq!(result, "/api/path?query=query");
}

#[test]
fn test_create_url_from_dataset_min_plus_query_and_fields() {
    let dataset = Dataset {
        path: "path".into(),
        resource: None,
        query: Some("query".into()),
        sub_resource: None,
        fields: Fields {
            fields: vec![Field { id: "id".into() }],
        },
    };
    let result = create_url_from_dataset(&dataset, "");
    assert_eq!(result, "/api/path?query=query&fields=id");
}

#[test]
fn test_create_url_from_dataset_other() {
    assert_eq!(
        "/api/path/resource",
        create_url_from_dataset(
            &Dataset {
                path: "path".into(),
                resource: Some("resource".into()),
                query: None,
                sub_resource: None,
                fields: Fields { fields: vec![] },
            },
            "",
        )
    );
    // With resource, query is not used
    assert_eq!(
        "/api/path/resource",
        create_url_from_dataset(
            &Dataset {
                path: "path".into(),
                resource: Some("resource".into()),
                query: Some("query".into()),
                sub_resource: None,
                fields: Fields { fields: vec![] },
            },
            "",
        )
    );
    assert_eq!(
        "/api/path/resource/sub",
        create_url_from_dataset(
            &Dataset {
                path: "path".into(),
                resource: Some("resource".into()),
                query: Some("query".into()),
                sub_resource: Some("sub".into()),
                fields: Fields { fields: vec![] },
            },
            "",
        )
    );
}

pub fn mock_handle_response(
    _config: &RiteYoutrackImport,
    _handler: &mut dyn RecordHandler,
    _response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn mock_config() -> RiteYoutrackImport {
    let config = RiteYoutrackImport {
        dataset: Dataset {
            path: "path".into(),
            resource: None,
            query: None,
            sub_resource: None,
            fields: Fields { fields: vec![] },
        },
    };
    config
}

#[test]
fn test_make_request_success() {
    let mut server = mockito::Server::new();
    let _m = server.mock("GET", "/api/path").with_status(200).create();

    let config = mock_config();
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);

    let url = server.url();
    let token = "test-token";

    let result = make_request(&mut handler, &config, &url, token, mock_handle_response);

    assert!(result.is_ok());
}

#[test]
fn test_make_request_error() {
    let mut server = mockito::Server::new();
    let _m = server
        .mock("GET", "/api/path")
        .with_status(500)
        .with_body("{}")
        .create();

    let config = mock_config();
    let mut records = Vec::new();
    let mut handler = CollectingRecordHandler::new(&mut records);

    let url = server.url();
    let token = "test-token";

    let result = make_request(&mut handler, &config, &url, token, mock_handle_response);
    // println!("{:?}", result);
    assert!(result.is_err_and(|e| e.to_string() == "null: null"));
}
