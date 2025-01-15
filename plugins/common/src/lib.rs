use console::ConsoleExporter;
use transformer::CommonTransformer;

pub mod console;
pub mod transformer;

/// Plugin entry function to create an instance of an [Transformer]
#[no_mangle]
pub fn create_transformer(
    _name: &str,
) -> Result<Box<dyn transform::Transformer>, Box<dyn std::error::Error>> {
    Ok(Box::new(CommonTransformer::new()))
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
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}
