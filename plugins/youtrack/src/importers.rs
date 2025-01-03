use config::RiteYoutrackImport;
use import::Importer;
use model::{field::Field, record::Record, xml::file::load_and_substitute_from_env, Initializable};
use serde_json::Value;
use youtrack::IssueWorkItem;

static CFG_URL: &str = "url";
static CFG_TOKEN: &str = "token";

pub struct YouTrackImporter {
    token: Option<String>,
    url: Option<String>,
    xml_config: Option<RiteYoutrackImport>,
}

impl YouTrackImporter {
    pub fn new() -> Self {
        YouTrackImporter {
            token: None,
            url: None,
            xml_config: None,
        }
    }

    /// Checks if all mandatory configuration options have a value
    fn check_config(&self) -> Option<&str> {
        self.url
            .is_none()
            .then_some(CFG_URL)
            .or_else(|| self.token.is_none().then_some(CFG_TOKEN))
    }

    fn read_from_youtrack(
        &mut self,
        callback: import::RecordCallback,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref base_url) = self.url {
            if let Some(ref token) = self.token {
                if let Some(ref xml_config) = self.xml_config {
                    match xml_config.dataset.path.as_str() {
                        "issues" => {
                            handle_issues(callback, &xml_config, &base_url, &token)?;
                        }
                        _ => {
                            return Err(format!("Unknown path '{}'", xml_config.dataset.path).into())
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn handle_issues(
    callback: import::RecordCallback,
    xml_config: &RiteYoutrackImport,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match xml_config.dataset.sub_resource.as_str() {
        "timeTracking/workItems" => {
            handle_issue_time_tracking(callback, &xml_config, &base_url, &token)?;
        }
        _ => {
            return Err(
                format!("Unknown sub-resource '{}'", xml_config.dataset.sub_resource).into(),
            )
        }
    }

    Ok(())
}

fn handle_issue_time_tracking(
    callback: import::RecordCallback,
    xml_config: &RiteYoutrackImport,
    base_url: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "{}/api/{}/{}/{}?fields={}",
        base_url,
        xml_config.dataset.path,
        xml_config.dataset.resource,
        xml_config.dataset.sub_resource,
        xml_config.dataset.fields
    );

    let response = client.get(url).bearer_auth(token).send()?;
    let status = response.status();
    if status.is_success() {
        handle_response(callback, response)?;
    } else {
        let error_for_status_ref = response.error_for_status_ref();
        if let Err(e) = error_for_status_ref {
            return Err(e.into());
        }
    }

    Ok(())
}

fn handle_response(
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    match response.json::<Value>() {
        Ok(result) => {
            if let Some(array) = result.as_array() {
                for element in array {
                    if let Ok(work_item) = serde_json::from_value::<IssueWorkItem>(element.clone())
                    {
                        handle_work_item(callback, work_item);
                    }
                }
            } else {
                return Err("Response is not a JSON Array".into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

fn handle_work_item(callback: import::RecordCallback, work_item: IssueWorkItem) {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    fields.push(Field::new_value(
        "date".to_string(),
        model::value::Value::I64(work_item.date),
    ));
    fields.push(Field::new_value(
        "work_item_id".to_string(),
        model::value::Value::String(work_item.id),
    ));
    fields.push(Field::new_value(
        "user_id".to_string(),
        model::value::Value::String(work_item.author.id),
    ));
    fields.push(Field::new_value(
        "user_name".to_string(),
        model::value::Value::String(work_item.author.name),
    ));
    fields.push(Field::new_value(
        "duration_minutes".to_string(),
        model::value::Value::I32(work_item.duration.minutes),
    ));
    callback(&record);
}

impl Importer for YouTrackImporter {
    fn read(&mut self, callback: import::RecordCallback) -> Result<(), Box<dyn std::error::Error>> {
        match self.check_config() {
            Some(variable) => {
                // Some configuration variable is missing
                return Err(format!("Configuration key '{}' is missing", variable).into());
            }
            None => {
                // Everything is ok
                self.read_from_youtrack(callback)?;
            }
        }

        Ok(())
    }

    /// Reset is currently not supported
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Initializable for YouTrackImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(url) = config.get(CFG_URL) {
                self.url = Some(String::from(url));
            }
            if let Some(token) = config.get(CFG_TOKEN) {
                self.token = Some(String::from(token));
            }

            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let xml_config: config::RiteYoutrackImport =
                            serde_xml_rs::from_str(&xml_contents)?;
                        self.xml_config = Some(xml_config);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
}

mod config;
mod youtrack;

#[cfg(test)]
mod tests;
