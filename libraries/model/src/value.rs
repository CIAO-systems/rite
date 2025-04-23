//! Module for the Value
//!
use chrono::NaiveDate;
use std::fmt::Display;

pub mod from;

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
    Collection(Vec<Value>),
    Record(crate::record::Record),
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
            Value::F32(val) => write!(f, "{:.}", val),
            Value::F64(val) => write!(f, "{:.}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Date(val) => write!(f, "{}", val),
            Value::Blob(vec) => {
                // Displaying bytes as hexadecimal
                let hex: Vec<String> = vec.iter().map(|b| format!("{:02x}", b)).collect();
                write!(f, "[{}]", hex.join(", "))
            }
            Value::Collection(collection) => {
                let values: Vec<String> = collection.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", values.join(", "))
            }
            Value::Record(record) => {
                let fields: Vec<String> = record
                    .fields()
                    .iter()
                    .map(|f| format!("{}={}", f.name(), f.value().to_string()))
                    .collect();
                write!(f, "{{{}}}", fields.join(", "))
            }
            Value::None => write!(f, "<None>"),
        }
    }
}

#[cfg(test)]
mod tests;
