use super::config::Configuration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Exporters {
    #[serde(rename = "exporter")]
    pub exporters: Vec<Exporter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exporter {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<Configuration>,
}
