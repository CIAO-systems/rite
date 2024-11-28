use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rite {
    pub plugins: Plugins,
    pub processes: Processes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "id")]
    pub id: String,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Processes {
    #[serde(rename = "process")]
    pub processes: Vec<Process>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "id")]
    pub id: String,
    pub importer: Importer,
    pub transformers: Transformers,
    pub exporters: Exporters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Importer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: String,
    pub configuration: Option<Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "config", deserialize_with = "deserialize_config_hashmap")]
    pub configs: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformers {
    #[serde(rename = "transformer")]
    pub transformers: Vec<Transformer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: String,
    pub configuration: Option<Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exporters {
    #[serde(rename = "exporter")]
    pub exporters: Vec<Exporter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exporter {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: String,
    pub configuration: Option<Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigItem {
    pub key: String,
    pub value: String,
}

// Custom deserialization function to convert config items to a HashMap
fn deserialize_config_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let configs = Vec::<ConfigItem>::deserialize(deserializer)?;
    Ok(configs
        .into_iter()
        .map(|config| (config.key, config.value))
        .collect())
}

pub mod file;

#[cfg(test)]
mod test;
