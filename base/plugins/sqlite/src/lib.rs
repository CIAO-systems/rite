use crate::importer::SQLiteImporter;

mod importer;

/// This function creates an importer for data in a SQLite database
///
#[unsafe(no_mangle)]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(SQLiteImporter::new()))
}

#[cfg(test)]
mod tests {
    use crate::create_importer;

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
}