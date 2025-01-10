use model::record::Record;
use serde::{Deserialize, Serialize};

use crate::importers::youtrack::factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub name: Option<String>,
}

impl From<Project> for Record {
    fn from(value: Project) -> Self {
        factory::serialize_to_record(value)
    }
}
