use serde::{Deserialize, Serialize};

pub mod config;
pub mod file;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rite {
    pub plugins: Plugins,
    pub processes: Processes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "id")]
    pub id: String,
    pub path: Option<String>,
    pub name: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Importer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<config::Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformers {
    #[serde(rename = "transformer")]
    pub transformers: Vec<Transformer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<config::Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exporters {
    #[serde(rename = "exporter")]
    pub exporters: Vec<Exporter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exporter {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<config::Configuration>,
}

#[cfg(test)]
mod test;
