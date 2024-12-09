use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub xml: Option<String>,

    #[serde(rename = "config")]
    pub config: Option<Vec<ConfigItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigItem {
    pub key: String,
    pub value: String,
}
impl ConfigItem {
    fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            xml: None,
            config: Some(Vec::new()),
        }
    }

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

    /// Adds a new key to the map with the given value
    ///
    pub fn insert(&mut self, key: String, value: String) {
        if let Some(ref mut config) = self.config {
            config.push(ConfigItem::new(key, value));
        }
    }
}
