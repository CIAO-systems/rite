use import::Importer;
use model::{field::Field, record::Record, Initializable};

static CFG_URL: &str = "url";
static CFG_TOKEN: &str = "token";

pub struct YouTrackImporter {
    token: Option<String>,
    url: Option<String>,
}

impl YouTrackImporter {
    pub fn new() -> Self {
        YouTrackImporter {
            token: None,
            url: None,
        }
    }

    /// Checks if all mandatory configuration options have a value
    fn check_config(&self) -> Option<&str> {
        self.url
            .is_none()
            .then_some(CFG_URL)
            .or_else(|| self.token.is_none().then_some(CFG_TOKEN))
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
                let mut record = Record::new();
                record.fields_as_mut().push(Field::new_value(
                    "name".to_string(),
                    model::value::Value::String("The Name".to_string()),
                ));
                callback(&record);
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
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
