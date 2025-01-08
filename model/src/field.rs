//! Module for the Field
//!
use crate::value::Value;

/// A struct to represent a field in a record
///
/// # Members
/// * `name` - Name of the field
/// * `value` - The value of the field
///
#[derive(Debug, Clone)]
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
    /// let field = model::field::Field::new("name".to_string());
    /// println!("{}", field.name());
    /// ```
    pub fn new(name: String) -> Self {
        Field {
            name,
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
    /// let field = model::field::Field::new_value("name".to_string(),
    ///     model::value::Value::I32(73));
    /// println!("{}", field.name());
    /// println!("{}", field.value());
    /// ```
    pub fn new_value(name: String, value: Value) -> Self {
        Field { name, value }
    }

    /// Creates a new boolean [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [bool] of the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_bool("name".to_string(),
    ///     false);
    /// if let model::value::Value::Bool(value) = field.value() {
    ///     println!("{} is true", field.name());
    /// }
    /// ```
    pub fn new_bool(name: String, value: bool) -> Self {
        Field {
            name,
            value: Value::Bool(value),
        }
    }

    /// Creates a new i32 [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [i32] of the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_i32("name".to_string(),
    ///     73);
    /// if let model::value::Value::I32(value) = field.value() {
    ///     println!("{} is an integer with value {}", field.name(), field.value());
    /// }
    /// ```
    pub fn new_i32(name: String, value: i32) -> Self {
        Field {
            name,
            value: Value::I32(value),
        }
    }

    /// Creates a new usize [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [usize] of the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_usize("name".to_string(),
    ///     42);
    /// if let model::value::Value::USize(value) = field.value() {
    ///     println!("{} is a usize with value {}", field.name(), field.value());
    /// }
    /// ```
    pub fn new_usize(name: String, value: usize) -> Self {
        Field {
            name,
            value: Value::USize(value),
        }
    }

    /// Creates a new float [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [f64] of the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_f64("name".to_string(),
    ///     42.73);
    /// if let model::value::Value::F64(value) = field.value() {
    ///     println!("{} is a float with value {}", field.name(), field.value());
    /// }
    /// ```
    pub fn new_f64(name: String, value: f64) -> Self {
        Field {
            name,
            value: Value::F64(value),
        }
    }

    /// Creates a new string [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [String] of the new field
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new_string("name".to_string(),
    ///     String::from("Chuck Norris"));
    /// if let model::value::Value::String(value) = field.value() {
    ///     println!("His {} is {}", field.name(), field.value());
    /// }
    /// ```
    pub fn new_string(name: String, value: String) -> Self {
        Field {
            name,
            value: Value::String(value),
        }
    }

    /// Creates a new binary [Field] with the given `name` and `value`
    ///
    /// # Arguments
    /// * `name` -  The name for the new field
    /// * `value` -  The [Vec] of the new field
    ///
    /// # Example
    /// ```
    /// let blob = vec![0x00, 0x01, 0x02, 0x03];
    /// let field = model::field::Field::new_blob("data".to_string(), blob.clone());
    /// if let model::value::Value::Blob(value) = field.value() {
    ///     println!("{} = {:?}", field.name(), field.value());
    /// }
    /// ```
    pub fn new_blob(name: String, value: Vec<u8>) -> Self {
        Field {
            name,
            value: Value::Blob(value),
        }
    }

    /// Returns the name of the [Field]
    ///
    /// # Example
    /// ```
    /// let field = model::field::Field::new("name".to_string());
    /// println!("{}", field.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// # Example
    /// ```
    /// let field = model::field::Field::new_f64("name".to_string(),
    ///     73.42);
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
    /// let field = model::field::Field::new_bool("is_active".to_string(), true);
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
        Field::new("default".to_string())
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
    fields.push(Field::new_value(name.to_string(), value));
}

#[cfg(test)]
mod tests;
