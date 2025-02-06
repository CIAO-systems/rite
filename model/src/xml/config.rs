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
mod tests {
    use super::*;

    #[test]
    fn test_get_config_value_success() {
        let mut config = Configuration::new();
        config.insert("port".to_string(), "8080".to_string());

        let port: Option<u16> = get_config_value(&Some(config.clone()), "port");
        assert_eq!(port, Some(8080));

        let value: Option<String> = get_config_value(&Some(config.clone()), "port");
        assert_eq!(value, Some("8080".to_string()));

        let enabled: Option<bool> = get_config_value(&Some(config), "enabled"); // Not present
        assert_eq!(enabled, None);

        let mut config = Configuration::new();
        config.insert("enabled".to_string(), "true".to_string());
        let enabled: Option<bool> = get_config_value(&Some(config), "enabled"); // Present
        assert_eq!(enabled, Some(true));
    }

    #[test]
    fn test_get_config_value_missing_key() {
        let config = Configuration::new();
        let value: Option<String> = get_config_value(&Some(config), "missing_key");
        assert_eq!(value, None);
    }

    #[test]
    fn test_get_config_value_empty_config() {
        let value: Option<String> = get_config_value(&None, "any_key");
        assert_eq!(value, None);
    }

    #[test]
    fn test_get_config_value_invalid_type() {
        let mut config = Configuration::new();
        config.insert("port".to_string(), "not_a_number".to_string());

        let port: Option<u16> = get_config_value(&Some(config), "port");
        assert_eq!(port, None); // Parsing should fail

        let mut config = Configuration::new();
        config.insert("float".to_string(), "3.14".to_string());
        let float_val: Option<f32> = get_config_value(&Some(config), "float");
        assert_eq!(float_val, Some(3.14));
    }

    #[test]
    fn test_get_config_value_with_spaces() {
        let mut config = Configuration::new();
        config.insert(
            "key_with_spaces".to_string(),
            "  value with spaces  ".to_string(),
        );

        let value: Option<String> = get_config_value(&Some(config), "key_with_spaces");
        assert_eq!(value, Some("  value with spaces  ".to_string())); // Preserves spaces

        let mut config = Configuration::new();
        config.insert("key_with_spaces_trimmed".to_string(), "  123  ".to_string());
        let value_trimmed: Option<u32> = get_config_value(&Some(config), "key_with_spaces_trimmed");
        assert_eq!(value_trimmed, None); // spaces dont parse
    }
}
