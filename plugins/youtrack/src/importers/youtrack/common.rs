use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DurationValue {
    #[serde(rename = "$type")]
    #[serde(default)]
    pub object_type: String,
    pub id: String,
    pub minutes: i32,
    pub presentation: String,
}
