use chrono::NaiveDate;

use crate::record::Record;

use super::Value;

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
