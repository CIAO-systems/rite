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
            Value::I128(v) => mysql::Value::Double(v as f64),
            Value::ISize(v) => mysql::Value::Int(v as i64),

            // --- Unsigned integers ---
            Value::U8(v) => mysql::Value::UInt(v as u64),
            Value::U16(v) => mysql::Value::UInt(v as u64),
            Value::U32(v) => mysql::Value::UInt(v as u64),
            Value::U64(v) => mysql::Value::UInt(v),
            Value::U128(v) => mysql::Value::Double(v as f64),
            Value::USize(v) => mysql::Value::UInt(v as u64),

            // --- Floating-point numbers ---
            Value::F32(v) => mysql::Value::Float(v),
            Value::F64(v) => mysql::Value::Double(v),

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
            // Unsupported types
            // --- Decimal ---
            // Value::Decimal(d) => mysql::Value::Bytes(d.to_string().into_bytes()),
            _ => mysql::Value::NULL,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use rust_decimal::dec;

    use crate::exporter::sql::value::ValueWrapper;

    #[test]
    fn test_from_bool() {
        let v = ValueWrapper(model::value::Value::Bool(true));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Int(1));
    }

    #[test]
    fn test_from_char() {
        let v = ValueWrapper(model::value::Value::Char('A'));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Bytes("A".as_bytes().to_vec()));
    }

    #[test]
    fn test_from_int() {
        let v = ValueWrapper(model::value::Value::I8(8));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Int(8));

        let v = ValueWrapper(model::value::Value::I16(-16));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Int(-16));

        let v = ValueWrapper(model::value::Value::I32(32));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Int(32));

        let v = ValueWrapper(model::value::Value::I64(-64));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Int(-64));

        let v = ValueWrapper(model::value::Value::I128(128));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Double(128.0));

        let v = ValueWrapper(model::value::Value::ISize(-256));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Int(-256));
    }

    #[test]
    fn test_from_uint() {
        let v = ValueWrapper(model::value::Value::U8(8));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::UInt(8));

        let v = ValueWrapper(model::value::Value::U16(16));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::UInt(16));

        let v = ValueWrapper(model::value::Value::U32(32));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::UInt(32));

        let v = ValueWrapper(model::value::Value::U64(64));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::UInt(64));

        let v = ValueWrapper(model::value::Value::U128(128));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Double(128.0));

        let v = ValueWrapper(model::value::Value::USize(256));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::UInt(256));
    }

    #[test]
    fn test_from_float() {
        let v = ValueWrapper(model::value::Value::F32(84.72));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Float(84.72));

        let v = ValueWrapper(model::value::Value::F64(84.72));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Double(84.72));
    }

    #[test]
    fn test_from_string() {
        const EXPECTED: &str = "I am your father";
        let v = ValueWrapper(model::value::Value::String(EXPECTED.into()));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Bytes(EXPECTED.as_bytes().to_vec()));
    }

    #[test]
    fn test_from_blob() {
        const EXPECTED: &str = "I am your father";
        let v = ValueWrapper(model::value::Value::Blob(EXPECTED.into()));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Bytes(EXPECTED.into()));
    }

    #[test]
    fn test_from_temporal() {
        let expected = NaiveDate::from_ymd_opt(1991, 11, 24).unwrap();
        let v = ValueWrapper(model::value::Value::Date(expected));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Date(1991, 11, 24, 0, 0, 0, 0));

        let v = ValueWrapper(model::value::Value::DateTime(
            expected.and_hms_opt(12, 13, 14).unwrap(),
        ));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Date(1991, 11, 24, 12, 13, 14, 0));

        let v = ValueWrapper(model::value::Value::Time(
            NaiveTime::from_hms_opt(12, 13, 14).unwrap(),
        ));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::Time(false, 0, 12, 13, 14, 0));
    }

    #[test]
    fn test_from_null() {
        let v = ValueWrapper(model::value::Value::None);
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::NULL);
    }

    #[test]
    fn test_from_decimal() {
        let v = ValueWrapper(model::value::Value::Decimal(dec!(84.72)));
        let mv: mysql::Value = v.into();
        assert_eq!(mv, mysql::Value::NULL); // Not supported -> NULL
    }
}
