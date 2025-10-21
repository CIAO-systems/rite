use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-sqlite-import")]
pub struct RiteSQLiteImport {
    pub filename: String,
    pub sql: String,
}

#[cfg(test)]
mod tests;
