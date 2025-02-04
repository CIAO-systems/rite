use config::{RiteYoutrackImportTime, TimeTracking};
use import::{Importer, RecordHandler};
use model::Initializable;

use crate::importers::connection::YouTrackConnection;

mod config;
mod rest;

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
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref time_tracking) = self.time_tracking {
            if let Some(ref connection) = self.connection {
                if let Some(ref url) = connection.url {
                    if let Some(ref token) = connection.token {
                        rest::request::make_request(
                            handler,
                            time_tracking,
                            &url,
                            &token,
                            rest::response::handle,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
