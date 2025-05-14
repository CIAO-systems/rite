use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

use super::Field;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Mapper {
    #[serde(rename = "field")]
    fields: Vec<Field>,
}

impl Mapper {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn Error>> {
        // 1. Read the file content as a string
        let xml_string = fs::read_to_string(file_name)?;

        // 2. Deserialize the XML string using serde
        let data: Self = serde_xml_rs::from_str(&xml_string)?;
        Ok(data)
    }

    pub fn get(&self, key: String) -> Option<Field> {
        self.fields
            .iter()
            .find(|field| field.name.source == key)
            .cloned()
    }
}

#[cfg(test)]
mod tests;