use model::import::RecordHandler;
use serde_json::Value;

use crate::importers::generic::config::{Dataset, Fields, RiteYoutrackImport};

// Define the type alias for the response handler function signature
type ResponseHandler = fn(
    config: &RiteYoutrackImport,
    handler: &mut dyn RecordHandler,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>>;

fn fields_str(fields: &Fields, sep: char) -> String {
    let fs = fields.to_string();
    if fs.is_empty() {
        "".into()
    } else {
        format!("{sep}fields={fs}")
    }
}

/// Create a URL string from the dataset values
pub fn create_url_from_dataset(dataset: &Dataset, base_url: &str) -> String {
    let url = if let Some(ref resource) = dataset.resource {
        if let Some(ref sub_resource) = dataset.sub_resource {
            format!(
                "{}/api/{}/{}/{}{}",
                base_url,
                dataset.path,
                resource,
                sub_resource,
                fields_str(&dataset.fields, '?')
            )
        } else {
            format!(
                "{}/api/{}/{}{}",
                base_url,
                dataset.path,
                resource,
                fields_str(&dataset.fields, '?')
            )
        }
    } else {
        if let Some(ref query) = dataset.query {
            format!(
                "{}/api/{}?query={}{}",
                base_url,
                dataset.path,
                urlencoding::encode(query),
                fields_str(&dataset.fields, '&')
            )
        } else {
            format!(
                "{}/api/{}{}",
                base_url,
                dataset.path,
                fields_str(&dataset.fields, '?')
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

#[cfg(test)]
mod tests;
