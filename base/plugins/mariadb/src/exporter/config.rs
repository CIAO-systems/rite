use model::xml::common::{DatabaseConnection, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "rite-mariadb-export")]
pub struct RiteMariaDBExport {
    pub connection: DatabaseConnection,
    pub table: Table,
}
