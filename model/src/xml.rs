use plugin::Plugins;
use process::Processes;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod exporter;
pub mod file;
pub mod import;
pub mod plugin;
pub mod process;
pub mod transformer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rite {
    pub plugins: Plugins,
    pub processes: Processes,
}

#[cfg(test)]
mod test;
