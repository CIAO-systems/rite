use ::model::{export::Exporter, import::Importer};

pub mod config;
pub mod connection;
pub mod exporters;
pub mod importers;
pub mod model;

/// This functions creates an importer for CIAO data
///
#[no_mangle]
pub fn create_importer(name: &str) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
    match name {
        "absences" => Ok(Box::new(importers::absences::Absences::new())),
        "accounts" => Ok(Box::new(importers::accounts::Accounts::new())),
        "badges" => Ok(Box::new(importers::badges::Badges::new())),
        "clock_entries" => Ok(Box::new(importers::clock_entries::ClockEntries::new())),
        "cost_centers" => Ok(Box::new(importers::cost_centers::CostCenters::new())),
        "devices" => Ok(Box::new(importers::devices::Devices::new())),
        "projects" => Ok(Box::new(importers::projects::Projects::new())),
        "time_types" => Ok(Box::new(importers::time_types::TimeTypes::new())),
        _ => Err(format!("Importer '{name}' not found").into()),
    }
}

/// This functions creates an exporter for CIAO data
///
#[no_mangle]
pub fn create_exporter(name: &str) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
    match name {
        "absences" => Ok(Box::new(exporters::absences::Absences::new())),
        "badges" => Ok(Box::new(exporters::badges::Badges::new())),
        "accounts" => Ok(Box::new(exporters::accounts::Accounts::new())),
        "clock_entries" => Ok(Box::new(exporters::clock_entries::ClockEntries::new())),
        "cost_centers" => Ok(Box::new(exporters::cost_centers::CostCenters::new())),
        "projects" => Ok(Box::new(exporters::projects::Projects::new())),
        "project_tasks" => Ok(Box::new(exporters::project_tasks::ProjectTasks::new())),
        _ => Err(format!("Exporter '{name}' not found").into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_exporter, create_importer};
    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    fn test_create_importer_ok(name: &str) {
        let importer = create_importer(name);
        assert!(importer.is_ok());
        let importer = importer.unwrap();
        assert_eq!(
            type_of(&importer),
            "alloc::boxed::Box<dyn model::import::Importer>"
        );
    }

    fn test_create_exporter_ok(name: &str) {
        let exporter = create_exporter(name);
        assert!(exporter.is_ok());
        let exporter = exporter.unwrap();
        assert_eq!(
            type_of(&exporter),
            "alloc::boxed::Box<dyn model::export::Exporter>"
        );
    }

    #[test]
    fn test_create_importer() {
        test_create_importer_ok("absences");
        test_create_importer_ok("accounts");
        test_create_importer_ok("badges");
        test_create_importer_ok("clock_entries");
        test_create_importer_ok("cost_centers");
        test_create_importer_ok("devices");
        test_create_importer_ok("projects");
        test_create_importer_ok("time_types");

        let importer = create_importer("any");
        assert!(importer.is_err_and(|e| e.to_string() == "Importer 'any' not found"));
    }

    #[test]
    fn test_create_exporter() {
        test_create_exporter_ok("absences");
        test_create_exporter_ok("badges");
        test_create_exporter_ok("accounts");
        test_create_exporter_ok("clock_entries");
        test_create_exporter_ok("cost_centers");
        test_create_exporter_ok("projects");
        test_create_exporter_ok("project_tasks");

        let exporter = create_exporter("any");
        assert!(exporter.is_err_and(|e| e.to_string() == "Exporter 'any' not found"));
    }

}
