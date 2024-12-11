use serde::{Deserialize, Serialize};

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
