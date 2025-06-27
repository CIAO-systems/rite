use exporter::{console::ConsoleExporter, template::TemplateExporter};
use model::BoxedError;
use transformer::{common::CommonTransformer, mapper::MapperTransformer};

use crate::importer::env::EnvImporter;

pub mod exporter;
pub mod importer;
pub mod transformer;

/// This functions creates an importer for the given name
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn import::Importer>, BoxedError> {
    match name {
        "env" => Ok(Box::new(EnvImporter::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}

/// Plugin entry function to create an instance of an [Transformer]
#[unsafe(no_mangle)]
pub fn create_transformer(name: &str) -> Result<Box<dyn transform::Transformer>, BoxedError> {
    match name {
        "mapper" => Ok(Box::new(MapperTransformer::new())),
        _ => Ok(Box::new(CommonTransformer::new())),
    }
}

/// Plugin entry function to create an instance of an [Exporter]
/// # Arguments
/// * `name` - When the plugin supports multiple exporters, the `name` is used
///     to determined, what exporter to return
/// # Available exporters
/// * `console` - An [Exporter] to write to the stdout
#[unsafe(no_mangle)]
pub fn create_exporter(name: &str) -> Result<Box<dyn export::Exporter>, BoxedError> {
    match name {
        "console" => Ok(Box::new(ConsoleExporter::new())),
        "template" => Ok(Box::new(TemplateExporter::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}
