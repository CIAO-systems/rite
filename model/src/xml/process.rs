//! Module for the rite processes
use serde::{Deserialize, Serialize};

use super::{exporter::Exporters, import::Importer, transformer::Transformers};

/// A list of processes
#[derive(Debug, Serialize, Deserialize)]
pub struct Processes {
    #[serde(rename = "process")]
    pub processes: Vec<Process>,
}

/// A rite process (import->transform->export)
/// 
/// In a rite process, there can be only one [Importer]. The importer record 
/// will then be optionally fed to all the transformers. The transformed record, 
/// will then be given to every exporter.
/// 
/// # Members
/// * `id` - The unique id of this process
/// * `importer` - The import description
/// * `transformers` - An optional list of transformers
/// * `exporters` - A list of exporters
#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "id")]
    pub id: String,
    pub importer: Importer,
    pub transformers: Option<Transformers>,
    pub exporters: Exporters,
}
