//! Module for the Record
//!
use crate::{field::Field, value::Value};
use serde_json::Value as JsonValue;

/// A record struct, that is used to transfer data from the import data source
/// to the export data sink
///
/// # Members
/// * `fields` - a [Vec] of [Field]
///
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    fields: Vec<Field>,
}

impl Record {
    /// Creates a new [Record]
    ///
    /// # Example
    /// ```
    /// use model::record::Record;
    /// let record = Record::new();
    /// for field in record.fields() {
    ///     println!("{:?}", field);
    /// }
    /// ```
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    /// Creates a new [Record] that is a copy of the given `other`
    /// # Arguments
    /// * `other` - A [Record] to be copied from
    /// # Example
    /// ```
    /// use model::record::Record;
    /// use model::field::Field;
    /// let mut original = Record::new();
    /// original
    ///     .fields_as_mut()
    ///     .push(model::field::Field::new_value("value", model::value::Value::I32(42)));
    /// let copyied = Record::copy(&original);
    /// if let Some(field) = copyied.field_by_name("value") {
    ///     println!("{} = {}", field.name(), field.value());
    /// }
    /// ```
    ///
    pub fn copy(other: &Record) -> Self {
        Self {
            fields: other.fields.clone(),
        }
    }

    /// Returns a reference to the fields
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    /// Returns a mutable reference to the fields
    pub fn fields_as_mut(&mut self) -> &mut Vec<Field> {
        &mut self.fields
    }

    /// Returns a field by name. If the field cannot be found, a [None] is returned
    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == name)
    }
}

impl From<JsonValue> for Record {
    fn from(value: JsonValue) -> Self {
        let mut record = Record::new();
        if let JsonValue::Object(map) = value {
            let fields = record.fields_as_mut();
            for (key, json_value) in map {
                fields.push(Field::new_value(&key, Value::from(json_value)));
            }
        }
        record
    }
}

#[cfg(test)]
mod tests;
