//! The configuration element for all RITE elements (importer, transformer and exporter)
//!
use serde::{Deserialize, Serialize};

/// A struct for a configuration key/value list or a special XML file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    /// If the component needs a more complex configuration, this optional string
    /// can point to a XML file, that the component can use for its configuration
    pub xml: Option<String>,

    /// An optional list of [ConfigItem]s
    #[serde(rename = "config")]
    pub config: Option<Vec<ConfigItem>>,
}

/// A key/value configuration variable
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigItem {
    /// Name of the configuration variable
    pub key: String,

    /// A string value for this configuration variable
    pub value: String,
}
impl ConfigItem {
    /// Creates a new configuration variable
    /// # Arguments
    /// * `key` -  the name of the configuration variable
    /// * `value` - the string value of the configuration variable
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Configuration {
    /// Creates a new empty [Configuration]
    pub fn new() -> Self {
        Self {
            xml: None,
            config: Some(Vec::new()),
        }
    }

    /// Creates a new [Configuration] with an external `xml` file attribute
    /// # Arguments
    /// * `xml` - Path to a extra configuration XML
    /// 
    pub fn with_xml(xml: &str) -> Self {
        Self {
            xml: Some(String::from(xml)),
            config: Some(Vec::new()),
        }
    }

    /// Get the config value for `key`
    ///
    pub fn get(&self, key: &str) -> Option<String> {
        match self.config {
            Some(ref config) => config
                .iter()
                .find(|item| item.key == key)
                .map(|item| item.value.clone()),
            _ => None,
        }
    }

    /// Returns the amount of keys in this configuration
    pub fn len(&self) -> usize {
        match self.config {
            Some(ref config) => config.len(),
            None => 0,
        }
    }

    /// Returns the items
    pub fn as_vec_ref(&self) -> Option<&Vec<ConfigItem>> {
        self.config.as_ref()
    }

    /// Adds a new key to the map with the given value
    ///
    pub fn insert(&mut self, key: String, value: String) {
        if let Some(ref mut config) = self.config {
            config.push(ConfigItem::new(key, value));
        }
    }
}
