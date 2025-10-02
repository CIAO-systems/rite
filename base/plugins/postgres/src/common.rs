use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    #[serde(rename = "@host")]
    pub host: String,
    #[serde(rename = "@port")]
    pub port: u16,
    #[serde(rename = "@database")]
    pub database: String,
    #[serde(rename = "@user")]
    pub user: String,
    #[serde(rename = "@password")]
    pub password: String,
}
