//! Module for the rite transformer descriptions in XML
use super::config::Configuration;
use serde::{Deserialize, Serialize};

/// A list of all transformers
#[derive(Debug, Serialize, Deserialize)]
pub struct Transformers {
    #[serde(rename = "transformer")]
    pub transformers: Option<Vec<Transformer>>,
}

/// A transformer description.
///
/// # Members
/// * `plugin` - the id of the plugin from the plugins section
/// * `name` - the (optional) name of the transformer from the plugin 
///         (if there a more than one available)
/// * `configuration` - a [Configuration] element for this transformer
#[derive(Debug, Serialize, Deserialize)]
pub struct Transformer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<Configuration>,
}
