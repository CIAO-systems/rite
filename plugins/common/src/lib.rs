use console::ConsoleExporter;
use template::TemplateExporter;
use transformer::{common::CommonTransformer, mapper::MapperTransformer};

pub mod console;
pub mod transformer;
pub mod template;

/// Plugin entry function to create an instance of an [Transformer]
#[no_mangle]
pub fn create_transformer(
    name: &str,
) -> Result<Box<dyn transform::Transformer>, Box<dyn std::error::Error>> {
    match name {
        "mapper" => Ok(Box::new(MapperTransformer::new())),
        _ => Ok(Box::new(CommonTransformer::new()))
    }
}

/// Plugin entry function to create an instance of an [Exporter]
/// # Arguments
/// * `name` - When the plugin supports multiple exporters, the `name` is used
///     to determined, what exporter to return
/// # Available exporters
/// * `console` - An [Exporter] to write to the stdout
#[no_mangle]
pub fn create_exporter(
    name: &str,
) -> Result<Box<dyn export::Exporter>, Box<dyn std::error::Error>> {
    match name {
        "console" => Ok(Box::new(ConsoleExporter::new())),
        "template" => Ok(Box::new(TemplateExporter::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}
