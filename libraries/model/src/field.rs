//! Module for the Field
//!
use crate::value::Value;

/// A struct to represent a field in a record
///
/// # Members
/// * `name` - Name of the field
/// * `value` - The value of the field
///
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// Name of the field
    name: String,

    /// Value of the field
    value: Value,
}

impl Field {
    /// Creates a new [Field] with the given `name`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new("name");
    /// println!("{}", field.name());
    /// ```
    pub fn new(name: &str) -> Self {
        Field {
            name: name.to_string(),
            value: Value::None,
        }
    }

    /// Creates a new [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [Value] of the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_value("name",
    ///     model::value::Value::I32(73));
    /// println!("{}", field.name());
    /// println!("{}", field.value());
    /// ```
    pub fn new_value(name: &str, value: Value) -> Self {
        Field {
            name: name.to_string(),
            value,
        }
    }

    /// Returns the name of the [Field]
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new("name");
    /// println!("{}", field.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// # Example
    /// ```
    /// let field = model::field::Field::new_value("name",
    ///     model::value::Value::F64(73.42));
    /// match field.value() {
    ///     model::value::Value::F64(f) => println!("{} is a float with value {}", field.name(), f),
    ///     _ => println!("{:?}", field)
    /// }
    /// ```
    pub fn value(&self) -> Value {
        self.value.clone()
    }

    /// Returns the value as a reference
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_value("is_active", model::value::Value::Bool(true));
    /// match field.value_as_ref() {
    ///     model::value::Value::Bool(b) => if *b {
    ///         println!("true");
    ///     } else {
    ///         println!("false");
    ///     }
    ///     _ => panic!("Expected Bool value"),
    /// }
    /// ```
    pub fn value_as_ref(&self) -> &Value {
        &self.value
    }
}

/// Implements the [Default] trait by returning a new Field with name "default"
impl Default for Field {
    fn default() -> Self {
        Field::new("default")
    }
}

/// Adds a field, if value is [Some]
/// # Arguments
/// * `fields`: A vector of fields where the new field should be added
/// * `name`: The name for the new field
/// * `value`: An [Option] that contains the value
pub fn add_optional_field<T>(fields: &mut Vec<Field>, name: &str, value: Option<T>)
where
    T: Into<Value>,
{
    if let Some(value) = value {
        add_field(fields, name, value.into());
    }
}

/// Adds a field with the value of `value`
/// # Arguments
/// * `fields`: A vector of fields where the new field should be added
/// * `name`: The name for the new field
/// * `value`: The value for the new field
pub fn add_field(fields: &mut Vec<Field>, name: &str, value: Value) {
    fields.push(Field::new_value(name, value));
}

#[cfg(test)]
mod tests;
