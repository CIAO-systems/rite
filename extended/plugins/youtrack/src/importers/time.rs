use config::{RiteYoutrackImportTime, TimeTracking};
use model::Initializable;
use model::import::{Importer, RecordHandler};

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

#[cfg(test)]
mod tests {
    use model::{
        Initializable,
        import::{Importer, handlers::CollectingRecordHandler},
        xml::config::Configuration,
    };

    use crate::importers::{
        connection::{CFG_TOKEN, CFG_URL},
        time::YouTrackImporterTime,
    };

    #[test]
    fn test_init() {
        let mut importer = YouTrackImporterTime::new();
        let mut config = Configuration::with_xml("../../data/youtrack/tests/time.xml");
        config.insert_str(CFG_URL, "url");
        config.insert_str(CFG_TOKEN, "token");
        let result = importer.init(Some(config));
        assert!(result.is_ok());

        let connection = importer.connection.unwrap();
        assert_eq!(connection.token, Some("token".into()));
        assert_eq!(connection.url, Some("url".into()));
        assert!(importer.time_tracking.is_some());

        let time_tracking = importer.time_tracking.unwrap();
        assert_eq!(
            time_tracking.start_date_as_param(),
            Some("2025-01-01".into())
        );
        assert_eq!(time_tracking.end_date_as_param(), None);
    }

    #[test]
    fn test_init_err() {
        let mut importer = YouTrackImporterTime::new();
        let config = Configuration::with_xml("file-does-not-exist.xml");
        let result = importer.init(Some(config));
        assert!(result.is_ok());

        let connection = importer.connection.unwrap();
        assert_eq!(connection.token, None);
        assert_eq!(connection.url, None);
        assert!(importer.time_tracking.is_none());
    }

    #[test]
    fn test_import() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/api.*".into()))
            .with_body("[{\"field\": \"value\"}]")
            .with_status(200)
            .create();
        let url = server.url();

        let mut importer = YouTrackImporterTime::new();
        let mut config = Configuration::with_xml("../../data/youtrack/tests/time.xml");
        config.insert_str(CFG_URL, &url);
        config.insert_str(CFG_TOKEN, "token");
        let result = importer.init(Some(config));
        assert!(result.is_ok());
        let connection = importer.connection.clone().unwrap();
        assert_eq!(connection.url, Some(url.into()));

        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        let result = importer.read(&mut handler);
        // println!("{:?}", result);
        // println!("{:?}", records);
        assert!(result.is_ok());
        assert!(records.is_empty());

        server.reset();
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/api.*".into()))
            .with_body("[{\"$type\": \"IssueWorkItem\", \"id\": \"id\"}]")
            .with_status(200)
            .create();
        let mut records = Vec::new();
        let mut handler = CollectingRecordHandler::new(&mut records);
        let result = importer.read(&mut handler);
        // println!("{:?}", result);
        // println!("{:?}", records);
        assert!(result.is_ok());
        assert_eq!(records.len(), 1);
    }
}
