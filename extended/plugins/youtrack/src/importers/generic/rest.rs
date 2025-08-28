use model::import::RecordHandler;
use serde_json::Value;

use crate::importers::generic::config::{Dataset, RiteYoutrackImport};

// Define the type alias for the response handler function signature
type ResponseHandler = fn(
    config: &RiteYoutrackImport,
    handler: &mut dyn RecordHandler,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>>;

/// Create a URL string from the dataset values
pub fn create_url_from_dataset(dataset: &Dataset, base_url: &str) -> String {
    let url = if let Some(ref resource) = dataset.resource {
        if let Some(ref sub_resource) = dataset.sub_resource {
            format!(
                "{}/api/{}/{}/{}?fields={}",
                base_url,
                dataset.path,
                resource,
                sub_resource,
                dataset.fields.to_string()
            )
        } else {
            format!(
                "{}/api/{}/{}?fields={}",
                base_url,
                dataset.path,
                resource,
                dataset.fields.to_string()
            )
        }
    } else {
        if let Some(ref query) = dataset.query {
            format!(
                "{}/api/{}?query={}&fields={}",
                base_url,
                dataset.path,
                urlencoding::encode(query),
                dataset.fields.to_string()
            )
        } else {
            format!(
                "{}/api/{}?fields={}",
                base_url,
                dataset.path,
                dataset.fields.to_string()
            )
        }
    };
    url
}

pub fn make_request(
    handler: &mut dyn RecordHandler,
    xml_config: &RiteYoutrackImport,
    base_url: &str,
    token: &str,
    response_handler: ResponseHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = create_url_from_dataset(&xml_config.dataset, base_url);

    let response = client.get(url).bearer_auth(token).send()?;
    if !response.status().is_success() {
        let body: Value = response.json()?;
        return Err(format!("{}: {}", body["error"], body["error_description"]).into());
    } else {
        response_handler(xml_config, handler, response)?;
    }

    Ok(())
}
