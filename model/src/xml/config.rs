use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub xml: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_config_hashmap"
    )]
    pub config: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigItem {
    pub key: String,
    pub value: String,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            xml: None,
            config: Some(HashMap::new()),
        }
    }

    pub fn with_xml(xml: &str) -> Self {
        Self {
            xml: Some(String::from(xml)),
            config: Some(HashMap::new()),
        }
    }

    /// Get the config value for `key`
    ///
    pub fn get(&self, key: &str) -> Option<String> {
        match self.config {
            Some(ref config) => config.get(key).cloned(),
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
            config.insert(key, value);
        }
    }
}

// Custom deserialization function to convert config items to a HashMap
fn deserialize_config_hashmap<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    println!("deserialize_config_hashmap");

    let configs = <Vec<ConfigItem>>::deserialize(deserializer).map_err(serde::de::Error::custom)?;

    Ok(Some(
        configs
            .into_iter()
            .map(|config| (config.key, config.value))
            .collect(),
    ))
}
