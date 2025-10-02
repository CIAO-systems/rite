//! Example plugin for an importer

use model::import::Importer;
use text::TextFileImporter;

pub mod text;

/// Plugin entry function to create an instance of an [Importer]
/// # Arguments
/// * `name` - When the plugin supports multiple importers, the `name` is used 
///     to determined, what importer to return
/// # Available importers
/// * `text` - An [Importer] that reads lines from a text file
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
    match name {
        "text" => Ok(Box::new(TextFileImporter::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}
