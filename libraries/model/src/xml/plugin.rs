//! Module for the rite plugins
use serde::{Deserialize, Serialize};

/// A list of known plugins
#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

/// A rite plugin
/// # Members
/// * `id` - the unique id of this plugin referred to by an importer, an 
///     exporter or a transformer
/// * `path` - The OS path, where the plugin file is located
/// * `name` - The name of the file of the dynamic library, 
///     without platform specific parts (extension or prefixes). 
///     On Linux, the file `libplugin.so` would be referred here as `plugin`.
///     On macOS, the file `libplugin.dylib` would be referred here as `plugin`.
///     On Windows, the file `plugin.dll` would be referred here as `plugin`.
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "id")]
    pub id: String,
    pub path: Option<String>,
    pub name: String,
}
