use importers::{
    accounts::Accounts, clock_entries::ClockEntries, devices::Devices, projects::Projects,
    time_types::TimeTypes,
};

pub mod config;
pub mod connection;
pub mod importers;

/// This functions create an importer for CIAO data
///
#[no_mangle]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "accounts" => Ok(Box::new(Accounts::new())),
        "devices" => Ok(Box::new(Devices::new())),
        "projects" => Ok(Box::new(Projects::new())),
        "time_types" => Ok(Box::new(TimeTypes::new())),
        "clock_entries" => Ok(Box::new(ClockEntries::new())),
        _ => Err("Not implemented".into()),
    }
}
