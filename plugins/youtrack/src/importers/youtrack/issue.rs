use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub summary: Option<String>,
}
