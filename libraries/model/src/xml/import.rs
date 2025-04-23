//! Module for the rite import descriptions in XML
//! 
use super::config::Configuration;
use serde::{Deserialize, Serialize};

/// An importer description.
///
/// # Members
/// * `plugin` - the id of the plugin from the plugins section
/// * `name` - the (optional) name of the importer from the plugin 
///         (if there a more than one available)
/// * `configuration` - a [Configuration] element for this importer
#[derive(Debug, Serialize, Deserialize)]
pub struct Importer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<Configuration>,
}
