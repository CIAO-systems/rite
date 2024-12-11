use serde::{Deserialize, Serialize};

use super::{exporter::Exporters, import::Importer, transformer::Transformers};

#[derive(Debug, Serialize, Deserialize)]
pub struct Processes {
    #[serde(rename = "process")]
    pub processes: Vec<Process>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "id")]
    pub id: String,
    pub importer: Importer,
    pub transformers: Option<Transformers>,
    pub exporters: Exporters,
}
