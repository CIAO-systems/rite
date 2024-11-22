use crate::value::Value;

#[derive(Debug)]
pub struct Field {
    name: String,
    value: Value,
}

impl Field {
    pub fn new(name: String) -> Self {
        Field {
            name,
            value: Value::None,
        }
    }

    pub fn new_value(name: String, value: Value) -> Self {
        Field { name, value }
    }

    pub fn new_bool(name: String, value: bool) -> Self {
        Field {
            name,
            value: Value::Bool(value),
        }
    }

    pub fn new_i32(name: String, value: i32) -> Self {
        Field {
            name,
            value: Value::I32(value),
        }
    }

    pub fn new_usize(name: String, value: usize) -> Self {
        Field {
            name,
            value: Value::USize(value),
        }
    }

    pub fn new_f64(name: String, value: f64) -> Self {
        Field {
            name,
            value: Value::F64(value),
        }
    }

    pub fn new_string(name: String, value: String) -> Self {
        Field {
            name,
            value: Value::String(value),
        }
    }

    pub fn new_blob(name: String, value: Vec<u8>) -> Self {
        Field {
            name,
            value: Value::Blob(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> Value {
        self.value.clone()
    }

    pub fn value_as_ref(&self) -> &Value {
        &self.value
    }
}

#[cfg(test)]
mod tests;
