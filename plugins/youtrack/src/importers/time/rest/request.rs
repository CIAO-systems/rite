use crate::importers::time::config::TimeTracking;

/// Create a URL string from the time tracking values
pub fn create_url(time_tracking: &TimeTracking, base_url: &str) -> String {
    let mut query = String::from("fields=id");
    query.push_str(",created");
    query.push_str(",duration(minutes)");
    query.push_str(",author(id,email)");
    query.push_str(",issue(id,idReadable,summary,project(id,name))");

    if let Some(start) = time_tracking.start_date_as_param() {
        query.push_str(&format!("&startDate={}", start));
    }
    if let Some(end) = time_tracking.end_date_as_param() {
        query.push_str(&format!("&endDate={}", end));
    }

    format!("{base_url}/api/workItems?{query}")
}

/// Type alias for the response handler function signature
pub type TimeResponseHandler = fn(
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>>;

/// Calls the REST server for YouTrack work items
///
/// # Arguments
/// * `callback`: The callback for imported records
/// * `time_tracking_config`: The configuration for filtering the request
/// * `base_url`: The base URL of the YouTrack installation
/// * `token`: The bearer token for authenticating with YouTrack. See https://www.jetbrains.com/help/youtrack/server/manage-permanent-token.html
/// * `response_handler`: The function that processes the request response
pub fn make_request(
    callback: import::RecordCallback,
    time_tracking_config: &TimeTracking,
    base_url: &str,
    token: &str,
    response_handler: TimeResponseHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = create_url(&time_tracking_config, base_url);

    let response = client.get(url).bearer_auth(token).send()?;
    if !response.status().is_success() {
        let body: serde_json::Value = response.json()?;
        return Err(format!("{}: {}", body["error"], body["error_description"]).into());
    } else {
        // call the response handler
        response_handler(callback, response)?;
    }

    Ok(())
}
