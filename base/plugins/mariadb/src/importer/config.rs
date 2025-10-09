use model::xml::common::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-mariadb-import")]
pub struct RiteMariaDBImport {
    pub connection: DatabaseConnection,
    pub sql: String,
}

#[cfg(test)]
mod tests;
