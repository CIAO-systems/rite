//! Module for the rite export descriptions in XML

use super::config::Configuration;
use serde::{Deserialize, Serialize};

/// A list of all exporters
#[derive(Debug, Serialize, Deserialize)]
pub struct Exporters {
    #[serde(rename = "exporter")]
    pub exporters: Vec<Exporter>,
}

/// An exporter description.
///
/// # Members
/// * `plugin` - the id of the plugin from the plugins section
/// * `name` - the (optional) name of the exporter from the plugin 
///         (if there a more than one available)
/// * `configuration` - a [Configuration] element for this exporter
#[derive(Debug, Serialize, Deserialize)]
pub struct Exporter {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<Configuration>,
}
