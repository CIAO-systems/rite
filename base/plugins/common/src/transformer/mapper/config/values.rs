use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Values {
    pub value: Vec<Value>,
}

impl Values {
    pub fn get(&self, key: String) -> Option<Value> {
        self.value.iter().find(|value| value.source == key).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@target")]
    pub target: String,
}
