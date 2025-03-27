//! The configuration element for all RITE elements (importer, transformer and exporter)
//!
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::BoxedError;

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

    /// Get the config value or an error
    pub fn get_result(&self, key: &str) -> Result<String, BoxedError> {
        Ok(self
            .get(key)
            .ok_or(format!("Configuration key '{}' missing", key))?)
    }

    /// Get list value (comma separated values)
    pub fn get_list<I: FromStr>(&self, key: &str) -> Option<Vec<I>> {
        self.get(key)
            .map(|s| {
                s.split(',') // split string
                    .filter_map(|i| i.trim().parse::<I>().ok()) // trim and convert to I
                    .collect::<Vec<I>>()
            })
            .filter(|vec| !vec.is_empty())
    }

    /// Get boolean value
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|s| match s.to_lowercase().as_str() {
            "true" | "1" => Some(true),
            "false" | "0" => Some(false),
            _ => None,
        })
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
            // Try to find an existing item with the same name
            if let Some(item) = config.iter_mut().find(|item| item.key == key) {
                // Update existing item's value
                item.value = value.to_string();
            } else {
                // Add new item if no existing item found
                config.push(ConfigItem::new(key, value));
            }
        }
    }

    /// Adds a new key to the map with the given value. &str variant
    ///
    pub fn insert_str(&mut self, key: &str, value: &str) {
        self.insert(key.to_string(), value.to_string());
    }
}

/// Get a config value of type T or None, if not found or not parseable
/// # Arguments
/// * `config`: An optional [Configuration]
/// * `key`: The name of the [ConfigItem] to parse and return
///
pub fn get_config_value<T: std::str::FromStr>(
    config: &Option<Configuration>,
    key: &str,
) -> Option<T> {
    config
        .as_ref()
        .and_then(|c| c.get(key))
        .and_then(|v| v.parse::<T>().ok())
}

#[cfg(test)]
mod tests;
