use config::RiteYoutrackImport;
use import::Importer;
use model::{xml::file::load_and_substitute_from_env, Initializable};

static CFG_URL: &str = "url";
static CFG_TOKEN: &str = "token";

// Define the type alias for the response handler function signature
type ResponseHandler = fn(
    callback: import::RecordCallback,
    response: reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>>;

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
                            issues::handle_issues_path(callback, &xml_config, &base_url, &token)?;
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
mod issues;
mod rest;
mod youtrack;

#[cfg(test)]
mod tests;
