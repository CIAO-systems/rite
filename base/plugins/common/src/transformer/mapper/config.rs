use serde::{Deserialize, Serialize};

pub mod mapper;
pub mod pattern;
pub mod values;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub name: Name,
    #[serde(rename = "type")]
    pub field_type: Type,
    pub patterns: Option<pattern::Patterns>,
    pub values: Option<values::Values>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Name {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@target")]
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Type {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@target")]
    pub target: String,
}

#[cfg(test)]
mod tests;
