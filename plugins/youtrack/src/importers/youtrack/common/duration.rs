use model::record::Record;
use serde::{Deserialize, Serialize};

use crate::importers::youtrack::factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct DurationValue {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub minutes: i32,
    pub presentation: String,
}

impl From<DurationValue> for Record {
    fn from(value: DurationValue) -> Self {
        factory::json_to_record(value)
    }
}
