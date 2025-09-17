use exporter::{console::ConsoleExporter, template::TemplateExporter};
use model::BoxedError;
use transformer::{common::CommonTransformer, mapper::MapperTransformer};

use crate::importer::env::EnvImporter;

pub mod exporter;
pub mod importer;
pub mod transformer;

/// This function creates an importer for the given name
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn model::import::Importer>, BoxedError> {
    match name {
        "env" => Ok(Box::new(EnvImporter::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}

/// Plugin entry function to create an instance of an [Transformer]
#[unsafe(no_mangle)]
pub fn create_transformer(
    name: &str,
) -> Result<Box<dyn model::transform::Transformer>, BoxedError> {
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
pub fn create_exporter(name: &str) -> Result<Box<dyn model::export::Exporter>, BoxedError> {
    match name {
        "console" => Ok(Box::new(ConsoleExporter::new())),
        "template" => Ok(Box::new(TemplateExporter::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_exporter, create_importer, create_transformer};

    #[test]
    fn test_create_importer() {
        assert!(create_importer("unknown").is_err());
        let env = create_importer("env");
        assert!(env.is_ok());
    }

    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    #[test]
    fn test_type() {
        let s = "Hello";
        assert_eq!(type_of(&s), "&str");
    }

    #[test]
    fn test_create_transformer() {
        let unknown = create_transformer("unknown");
        assert!(
            unknown.is_ok_and(
                |t| type_of(&t) == "alloc::boxed::Box<dyn model::transform::Transformer>"
            )
        );

        let mapper = create_transformer("mapper");
        assert!(mapper.is_ok());
        let mapper = mapper.unwrap();
        assert_eq!(
            type_of(&mapper),
            "alloc::boxed::Box<dyn model::transform::Transformer>"
        );
    }

    #[test]
    fn test_create_exporter() {
        assert!(create_exporter("unknown").is_err());

        let console = create_exporter("console");
        assert!(console.is_ok());
        let console = console.unwrap();
        assert_eq!(
            type_of(&console),
            "alloc::boxed::Box<dyn model::export::Exporter>"
        );

        let template = create_exporter("template");
        assert!(template.is_ok());
        let template = template.unwrap();
        assert_eq!(
            type_of(&template),
            "alloc::boxed::Box<dyn model::export::Exporter>"
        );
    }
}
