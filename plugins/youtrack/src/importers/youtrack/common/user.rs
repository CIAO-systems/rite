use model::record::Record;
use serde::{Deserialize, Serialize};

use crate::importers::youtrack::factory;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub login: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    pub email: Option<String>,
}

impl From<User> for Record {
    fn from(value: User) -> Self {
        factory::json_to_record(value)
    }
}
