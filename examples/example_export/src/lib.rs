//! Example plugin for an exporter
use console::ConsoleExporter;
use export::Exporter;

/// Plugin entry function to create an instance of an [Exporter]
/// # Arguments
/// * `name` - When the plugin supports multiple exporters, the `name` is used 
///     to determined, what exporter to return
/// # Available exporters
/// * `console` - An [Exporter] to write to the stdout
#[no_mangle]
pub fn create_exporter(name: &str) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
    match name {
        "console" => Ok(Box::new(ConsoleExporter::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}

mod console;
