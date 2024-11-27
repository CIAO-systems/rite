use console::ConsoleExporter;
use export::Exporter;

#[no_mangle]
pub fn create_exporter(name: &str) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
    match name {
        "console" => Ok(Box::new(ConsoleExporter::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}

mod console;
