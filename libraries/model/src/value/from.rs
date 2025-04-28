use chrono::NaiveDate;

use super::Value;
use crate::record::Record;
use serde_json::Value as JsonValue;

// Implement Into<Value> for supported types
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value.to_string())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Char(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::I128(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::U128(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Blob(value)
    }
}

impl From<NaiveDate> for Value {
    fn from(value: NaiveDate) -> Self {
        Value::Date(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Collection(value)
    }
}

impl From<Record> for Value {
    fn from(value: Record) -> Self {
        Value::Record(value)
    }
}

impl From<JsonValue> for Value {
    fn from(json_value: JsonValue) -> Self {
        match json_value {
            JsonValue::Bool(b) => Value::Bool(b),
            JsonValue::Number(n) => {
                if n.is_i64() {
                    if let Some(i) = n.as_i64() {
                        if let Ok(i8_val) = i8::try_from(i) {
                            Value::I8(i8_val)
                        } else if let Ok(i16_val) = i16::try_from(i) {
                            Value::I16(i16_val)
                        } else if let Ok(i32_val) = i32::try_from(i) {
                            Value::I32(i32_val)
                        } else if let Ok(isize_val) = isize::try_from(i) {
                            Value::ISize(isize_val)
                        } else {
                            Value::I64(i)
                        }
                    } else {
                        Value::None
                    }
                } else if n.is_u64() {
                    if let Some(u) = n.as_u64() {
                        if let Ok(u8_val) = u8::try_from(u) {
                            Value::U8(u8_val)
                        } else if let Ok(u16_val) = u16::try_from(u) {
                            Value::U16(u16_val)
                        } else if let Ok(u32_val) = u32::try_from(u) {
                            Value::U32(u32_val)
                        } else if let Ok(usize_val) = usize::try_from(u) {
                            Value::USize(usize_val)
                        } else {
                            Value::U64(u)
                        }
                    } else {
                        Value::None
                    }
                } else if n.is_f64() {
                    if let Some(f) = n.as_f64() {
                        Value::F64(f)
                    } else {
                        Value::None
                    }
                } else {
                    Value::None
                }
            }
            JsonValue::String(s) => {
                if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                    Value::Date(date)
                } else if s.len() == 1 {
                    if let Some(c) = s.chars().next() {
                        Value::Char(c)
                    } else {
                        Value::String(s)
                    }
                } else {
                    Value::String(s)
                }
            }
            JsonValue::Array(arr) => {
                let collection: Vec<Value> = arr.into_iter().map(Value::from).collect();
                Value::Collection(collection)
            }
            JsonValue::Object(_) => Value::Record(Record::from(json_value)),
            JsonValue::Null => Value::None,
        }
    }
}

#[cfg(test)]
mod tests;