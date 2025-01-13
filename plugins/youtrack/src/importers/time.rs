use config::{RiteYoutrackImportTime, TimeTracking};
use import::Importer;
use model::{field::Field, record::Record, value::Value, Initializable};

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
        let mut record = Record::new();
        if let Some(ref time_tracking) = self.time_tracking {
            if let Some(start_date) = time_tracking.start_date {
                record.fields_as_mut().push(Field::new_value(
                    "start-date".to_string(),
                    Value::Date(start_date),
                ));
            }

            if let Some(end_date) = time_tracking.end_date {
                record.fields_as_mut().push(Field::new_value(
                    "end-date".to_string(),
                    Value::Date(end_date),
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
