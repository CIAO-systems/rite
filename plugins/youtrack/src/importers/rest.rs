use super::{config::{Dataset, RiteYoutrackImport}, ResponseHandler};

/// Create a URL string from the dataset values
pub fn create_url_from_dataset(dataset: &Dataset, base_url: &str) -> String {
    let url = if let Some(ref resource) = dataset.resource {
        if let Some(ref sub_resource) = dataset.sub_resource {
            format!(
                "{}/api/{}/{}/{}?fields={}",
                base_url, dataset.path, resource, sub_resource, dataset.fields
            )
        } else {
            format!(
                "{}/api/{}/{}?fields={}",
                base_url, dataset.path, resource, dataset.fields
            )
        }
    } else {
        if let Some(ref query) = dataset.query {
            format!(
                "{}/api/{}?query={}&fields={}",
                base_url, dataset.path, query, dataset.fields
            )
        } else {
            format!(
                "{}/api/{}?fields={}",
                base_url, dataset.path, dataset.fields
            )
        }
    };
    url
}

pub fn make_request(
    callback: import::RecordCallback,
    xml_config: &RiteYoutrackImport,
    base_url: &str,
    token: &str,
    response_handler: ResponseHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = create_url_from_dataset(&xml_config.dataset, base_url);

    let response = client.get(url).bearer_auth(token).send()?;
    let status = response.status();
    if status.is_success() {
        response_handler(callback, response)?;
    } else {
        let error_for_status_ref = response.error_for_status_ref();
        if let Err(e) = error_for_status_ref {
            return Err(e.into());
        }
    }

    Ok(())
}
