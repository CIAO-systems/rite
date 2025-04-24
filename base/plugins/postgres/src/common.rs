use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}
