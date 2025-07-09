use export::Exporter;

pub mod config;
pub mod connection;
pub mod exporters;
pub mod importers;
pub mod model;

/// This functions creates an importer for CIAO data
///
#[no_mangle]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "absences" => Ok(Box::new(importers::absences::Absences::new())),
        "accounts" => Ok(Box::new(importers::accounts::Accounts::new())),
        "clock_entries" => Ok(Box::new(importers::clock_entries::ClockEntries::new())),
        "cost_centers" => Ok(Box::new(importers::cost_centers::CostCenters::new())),
        "devices" => Ok(Box::new(importers::devices::Devices::new())),
        "projects" => Ok(Box::new(importers::projects::Projects::new())),
        "time_types" => Ok(Box::new(importers::time_types::TimeTypes::new())),
        _ => Err("Not implemented".into()),
    }
}

/// This functions creates an exporter for CIAO data
///
#[no_mangle]
pub fn create_exporter(name: &str) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
    match name {
        "absences" => Ok(Box::new(exporters::absences::Absences::new())),
        "accounts" => Ok(Box::new(exporters::accounts::Accounts::new())),
        "clock_entries" => Ok(Box::new(exporters::clock_entries::ClockEntries::new())),
        "cost_centers" => Ok(Box::new(exporters::cost_centers::CostCenters::new())),
        "projects" => Ok(Box::new(exporters::projects::Projects::new())),
        "project_tasks" => Ok(Box::new(exporters::project_tasks::ProjectTasks::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}
