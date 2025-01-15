//! Module for the Value
//!
use std::fmt::Display;

use chrono::NaiveDate;
/// An enum for all known field values.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    F32(f32),
    F64(f64),
    String(String),
    Blob(Vec<u8>),
    Date(NaiveDate),
    None,
}

/// Implements the [Display] trait for the [Value]
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(val) => write!(f, "{}", val),
            Value::Char(val) => write!(f, "{}", val),
            Value::I8(val) => write!(f, "{}", val),
            Value::I16(val) => write!(f, "{}", val),
            Value::I32(val) => write!(f, "{}", val),
            Value::I64(val) => write!(f, "{}", val),
            Value::I128(val) => write!(f, "{}", val),
            Value::ISize(val) => write!(f, "{}", val),
            Value::U8(val) => write!(f, "{}", val),
            Value::U16(val) => write!(f, "{}", val),
            Value::U32(val) => write!(f, "{}", val),
            Value::U64(val) => write!(f, "{}", val),
            Value::U128(val) => write!(f, "{}", val),
            Value::USize(val) => write!(f, "{}", val),
            Value::F32(val) => write!(f, "{}", val),
            Value::F64(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Date(val) => write!(f, "{}", val),
            Value::Blob(vec) => {
                // Displaying bytes as hexadecimal
                let hex: Vec<String> = vec.iter().map(|b| format!("{:02x}", b)).collect();
                write!(f, "[{}]", hex.join(", "))
            }
            Value::None => write!(f, "<None>"),
        }
    }
}

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
