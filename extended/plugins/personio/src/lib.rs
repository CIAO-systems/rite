pub mod importers;
mod macros;

/// This functions creates an importer for the Personio plugin
///
#[unsafe(no_mangle)]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "employees" => Ok(Box::new(importers::employees::Employees::new())),
        "projects" => Ok(Box::new(importers::projects::Projects::new())),
        "absences" => Ok(Box::new(importers::absences::Absences::new())),
        "attendances" => Ok(Box::new(importers::attendances::Attendances::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::create_importer;

    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    fn test_importer_with(name: &str) {
        let importer = create_importer(name);
        assert!(importer.is_ok());
        let importer = importer.unwrap();
        assert_eq!(
            type_of(&importer),
            "alloc::boxed::Box<dyn model::import::Importer>"
        );
    }

    #[test]
    fn test_create_importer() {
        test_importer_with("employees");
        test_importer_with("projects");
        test_importer_with("absences");
        test_importer_with("attendances");

        let importer = create_importer("this-is-not-a-known-importer");
        assert!(importer.is_err_and(|e| e.to_string() == "Unknown importer 'this-is-not-a-known-importer'" ));

    }
}
