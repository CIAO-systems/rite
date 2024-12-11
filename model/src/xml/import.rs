use super::config::Configuration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Importer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<Configuration>,
}
