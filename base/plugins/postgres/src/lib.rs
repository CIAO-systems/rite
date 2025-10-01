use exporter::PostgresExporter;
use importer::PostgresImporter;

mod common;
mod exporter;
mod importer;

#[unsafe(no_mangle)]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(PostgresImporter::new()))
}

#[unsafe(no_mangle)]
pub fn create_exporter(
    _name: &str,
) -> Result<Box<dyn model::export::Exporter>, Box<dyn std::error::Error>> {
    Ok(Box::new(PostgresExporter::new()))
}

#[cfg(test)]
mod embedded;

#[cfg(test)]
mod tests {
    use crate::{create_exporter, create_importer};

    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    #[test]
    fn test_create_importer() {
        let importer = create_importer("any");
        assert!(importer.is_ok());
        let importer = importer.unwrap();
        assert_eq!(
            type_of(&importer),
            "alloc::boxed::Box<dyn model::import::Importer>"
        );
    }

    #[test]
    fn test_create_exporter() {
        let exporter = create_exporter("any");
        assert!(exporter.is_ok());
        let porter = exporter.unwrap();
        assert_eq!(
            type_of(&porter),
            "alloc::boxed::Box<dyn model::export::Exporter>"
        );
    }
}
