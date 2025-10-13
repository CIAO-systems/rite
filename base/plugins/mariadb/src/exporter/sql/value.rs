use chrono::{Datelike, Timelike};
use model::value::Value;

#[derive(Clone, Debug)]
pub struct ValueWrapper(pub Value);

impl From<ValueWrapper> for mysql::Value {
    fn from(value: ValueWrapper) -> Self {
        match value.0 {
            // --- Boolean ---
            Value::Bool(b) => mysql::Value::Int(if b { 1 } else { 0 }),

            // --- Character ---
            Value::Char(c) => mysql::Value::Bytes(c.to_string().into_bytes()),

            // --- Signed integers ---
            Value::I8(v) => mysql::Value::Int(v as i64),
            Value::I16(v) => mysql::Value::Int(v as i64),
            Value::I32(v) => mysql::Value::Int(v as i64),
            Value::I64(v) => mysql::Value::Int(v),
            Value::I128(v) => mysql::Value::Bytes(v.to_string().into_bytes()),
            Value::ISize(v) => mysql::Value::Int(v as i64),

            // --- Unsigned integers ---
            Value::U8(v) => mysql::Value::UInt(v as u64),
            Value::U16(v) => mysql::Value::UInt(v as u64),
            Value::U32(v) => mysql::Value::UInt(v as u64),
            Value::U64(v) => mysql::Value::UInt(v),
            Value::U128(v) => mysql::Value::Bytes(v.to_string().into_bytes()),
            Value::USize(v) => mysql::Value::UInt(v as u64),

            // --- Floating-point numbers ---
            Value::F32(v) => mysql::Value::Float(v),
            Value::F64(v) => mysql::Value::Double(v),

            // --- Decimal ---
            Value::Decimal(d) => mysql::Value::Bytes(d.to_string().into_bytes()),

            // --- Strings ---
            Value::String(s) => mysql::Value::Bytes(s.into_bytes()),

            // --- Binary (BLOB) ---
            Value::Blob(bytes) => mysql::Value::Bytes(bytes),

            // --- Temporal types ---
            Value::Date(date) => mysql::Value::Date(
                date.year() as u16,
                date.month() as u8,
                date.day() as u8,
                0,
                0,
                0,
                0,
            ),
            Value::DateTime(dt) => mysql::Value::Date(
                dt.year() as u16,
                dt.month() as u8,
                dt.day() as u8,
                dt.hour() as u8,
                dt.minute() as u8,
                dt.second() as u8,
                dt.and_utc().timestamp_subsec_micros(),
            ),
            Value::Time(t) => mysql::Value::Time(
                false, // not negative duration
                0,     // days
                t.hour() as u8,
                t.minute() as u8,
                t.second() as u8,
                t.nanosecond() / 1000, // convert ns → µs
            ),

            // --- None/null ---
            Value::None => mysql::Value::NULL,
            _ => mysql::Value::NULL,
        }
    }
}
