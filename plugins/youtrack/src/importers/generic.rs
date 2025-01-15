use config::RiteYoutrackImport;
use super::youtrack::factory::fill_record_from_json;
use crate::importers::connection::YouTrackConnection;
use import::Importer;
use model::{record::Record, xml::file::load_and_substitute_from_env, Initializable};
use rest::make_request;

mod rest;
mod config;


pub struct YouTrackImporter {
    connection: Option<YouTrackConnection>,
    xml_config: Option<RiteYoutrackImport>,
}

impl YouTrackImporter {
    pub fn new() -> Self {
        YouTrackImporter {
            connection: None,
            xml_config: None,
        }
    }

    /// Sets the connection information
    pub fn set_connection(&mut self, config: &model::xml::config::Configuration) {
        self.connection = Some(YouTrackConnection::from(&config));
    }

    /// Checks if all mandatory configuration options have a value
    pub fn check_config(&self) -> Option<String> {
        if let Some(ref connection) = self.connection {
            connection.check_config()
        } else {
            Some(YouTrackConnection::all_variables())
        }
    }

    fn read_from_youtrack(
        &mut self,
        callback: import::RecordCallback,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref connection) = self.connection {
            if let (Some(ref base_url), Some(ref token)) =
                (connection.url.clone(), connection.token.clone())
            {
                if let Some(ref xml_config) = self.xml_config {
                    make_request(
                        callback,
                        xml_config,
                        base_url,
                        token,
                        YouTrackImporter::handle_response,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// A generic response handler for YouTrack datasets
    pub fn handle_response(
        _config: &RiteYoutrackImport,
        callback: import::RecordCallback,
        response: reqwest::blocking::Response,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = response.json::<serde_json::Value>()?;
        if let Some(array) = json.as_array() {
            for element in array {
                let mut record = Record::new();
                if fill_record_from_json(&mut record, element) {
                    callback(&record);
                }
            }
        } else if json.is_object() {
            let mut record = Record::new();
            if fill_record_from_json(&mut record, &json) {
                callback(&record);
            }
        }
        Ok(())
    }
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
            self.set_connection(&config);

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

#[cfg(test)]
mod tests;
