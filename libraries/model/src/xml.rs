//! Module for the XML configuration for RITE
//!
//! When reading the XML file, some variables will be replaced. In the XML all
//! system environment variables are available. Additionally, the following variables can be used:
//! * `RITE_CONFIG_PATH` - The path of the XML file (so other XML files can be addressed relative to the main file)
//! # Example
//! ```xml
//! <rite>
//!     <!-- ... -->
//!     <processes>
//!        <exporter plugin="postgres">
//!            <configuration xml="$RITE_CONFIG_PATH/postgres-config.xml" />
//!        </exporter>
//!     <!-- ... -->
//!     </processes>
//! </rite>
//! ```
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

/// The XML root element for a rite configuration
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Rite {
    /// A list of all uesed plugins for this rite
    ///
    pub plugins: Plugins,

    /// A list of processes (import -> transform -> export)
    pub processes: Processes,
}

#[cfg(test)]
mod test;
