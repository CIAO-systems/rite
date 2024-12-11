use super::config::Configuration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformers {
    #[serde(rename = "transformer")]
    pub transformers: Option<Vec<Transformer>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformer {
    #[serde(rename = "plugin")]
    pub plugin: String,
    pub name: Option<String>,
    pub configuration: Option<Configuration>,
}
