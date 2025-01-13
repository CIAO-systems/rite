use config::{RiteYoutrackImportTime, TimeTracking};
use import::Importer;
use model::{field::Field, record::Record, Initializable};

use crate::importers::connection::YouTrackConnection;

mod config;

pub struct YouTrackImporterTime {
    connection: Option<YouTrackConnection>,
    time_tracking: Option<TimeTracking>,
}

impl YouTrackImporterTime {
    pub(crate) fn new() -> Self {
        Self {
            connection: None,
            time_tracking: None,
        }
    }
}

impl Initializable for YouTrackImporterTime {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            self.connection = Some(YouTrackConnection::from(&config));

            if let Some(ref xml) = config.xml {
                self.time_tracking = match RiteYoutrackImportTime::from(xml) {
                    Ok(config) => Some(config.time_tracking),
                    Err(e) => {
                        eprintln!("Error while parsing configuration: {}", e);
                        log::error!("Error while parsing configuration: {}", e);
                        None
                    }
                }
            }
        }
        Ok(())
    }
}

impl Importer for YouTrackImporterTime {
    fn read(&mut self, callback: import::RecordCallback) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref time_tracking) = self.time_tracking {
            if let Some(ref connection) = self.connection {
                if let Some(ref url) = connection.url {
                    if let Some(ref token) = connection.token {
                        make_request(callback, time_tracking, &url, &token)?;
                    }
                }
            }
        }

        let mut record = Record::new();
        if let Some(ref time_tracking) = self.time_tracking {
            if let Some(start_date) = time_tracking.start_date {
                record.fields_as_mut().push(Field::new_value(
                    "start-date".to_string(),
                    model::value::Value::Date(start_date),
                ));
            }

            if let Some(end_date) = time_tracking.end_date {
                record.fields_as_mut().push(Field::new_value(
                    "end-date".to_string(),
                    model::value::Value::Date(end_date),
                ));
            }
        }
        callback(&record);

        Ok(())
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

/// Create a URL string from the dataset values
pub fn create_url_from_dataset(_time_tracking: &TimeTracking, base_url: &str) -> String {
    // format!(
    //     "{}/api/workItems?fields=id,created,duration(minutes)),author(id,email)&startDate={}",
    //     base_url, "2025-01-01",
    // )
    format!(
        "{}/api/workItems?fields=id,created,duration(minutes),author(id,email)&startDate=2025-01-01",
        base_url,
    )
}

pub fn make_request(
    callback: import::RecordCallback,
    time_tracking_config: &TimeTracking,
    base_url: &str,
    token: &str,
    // response_handler: ResponseHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = create_url_from_dataset(&time_tracking_config, base_url);

    let response = client.get(url).bearer_auth(token).send()?;
    if !response.status().is_success() {
        let body: serde_json::Value = response.json()?;
        return Err(format!("{}: {}", body["error"], body["error_description"]).into());
    } else {
        // call the response handler
        println!("{:#?}", response);

        let mut record = Record::new();
        if let Ok(text) = response.text() {
            record
                .fields_as_mut()
                .push(Field::new_string("response".to_string(), text));
        }

        callback(&record);
    }

    Ok(())
}
