use bytes::BytesMut;
use model::value::Value;
use postgres::types::{to_sql_checked, IsNull, ToSql, Type};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct ValueWrapper(pub Value);

impl ToSql for ValueWrapper {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match &self.0 {
            Value::Bool(v) => v.to_sql(ty, out),
            Value::Char(v) => v.to_string().to_sql(ty, out),
            Value::I8(v) => v.to_sql(ty, out),
            Value::I16(v) => v.to_sql(ty, out),
            Value::I32(v) => v.to_sql(ty, out),
            Value::I64(v) => (*v as i64).to_sql(ty, out),
            Value::I128(v) => (*v as i64).to_sql(ty, out),
            Value::ISize(v) => (*v as i64).to_sql(ty, out),
            Value::U8(v) => (*v as i64).to_sql(ty, out),
            Value::U16(v) => (*v as i64).to_sql(ty, out),
            Value::U32(v) => (*v as i64).to_sql(ty, out),
            Value::U64(v) => (*v as i64).to_sql(ty, out),
            Value::U128(v) => (*v as i64).to_sql(ty, out),
            Value::USize(v) => (*v as i64).to_sql(ty, out),
            Value::F32(v) => v.to_sql(ty, out),
            Value::F64(v) => v.to_sql(ty, out),
            Value::String(v) => v.to_sql(ty, out),
            Value::Blob(v) => v.to_sql(ty, out),
            Value::Date(v) => {
                let date_str = v.format("%Y-%m-%d").to_string();
                date_str.to_sql(ty, out)
            }
            _ => Ok(IsNull::Yes),
        }
    }

    fn accepts(ty: &Type) -> bool {
        matches!(
            ty,
            &Type::BOOL
                | &Type::VARCHAR
                | &Type::CHAR
                | &Type::INT2
                | &Type::INT4
                | &Type::INT8
                | &Type::FLOAT4
                | &Type::FLOAT8
                | &Type::TEXT
                | &Type::BYTEA
        )
    }

    to_sql_checked!();

    fn encode_format(&self, _ty: &Type) -> postgres::types::Format {
        postgres::types::Format::Binary
    }
}

pub fn _get_sql_type(value: &Value) -> &'static str {
    match value {
        Value::Bool(_) => "BOOLEAN",
        Value::Char(_) => "CHAR(1)",
        Value::I8(_) => "SMALLINT",
        Value::I16(_) => "SMALLINT",
        Value::I32(_) => "INTEGER",
        Value::I64(_) => "BIGINT",
        Value::I128(_) => "NUMERIC(39,0)",
        Value::ISize(_) => "INTEGER",
        Value::U8(_) => "SMALLINT",
        Value::U16(_) => "INTEGER",
        Value::U32(_) => "BIGINT",
        Value::U64(_) => "NUMERIC(20,0)",
        Value::U128(_) => "NUMERIC(39,0)",
        Value::USize(_) => "BIGINT",
        Value::F32(_) => "REAL",
        Value::F64(_) => "DOUBLE PRECISION",
        Value::String(_) => "TEXT",
        Value::Blob(_) => "BYTEA",
        Value::Date(_) => "DATE",
        _ => "TEXT", // Default for nullable column
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use chrono::Local;
    use model::value::Value;
    use postgres::types::{IsNull, ToSql, Type};

    use crate::exporter::sql::value::{ValueWrapper, _get_sql_type};

    #[test]
    fn test_get_sql_type() {
        assert_eq!(_get_sql_type(&Value::None), "TEXT");

        assert_eq!(_get_sql_type(&Value::Bool(true)), "BOOLEAN");
        assert_eq!(_get_sql_type(&Value::Char('c')), "CHAR(1)");
        assert_eq!(_get_sql_type(&Value::I8(1)), "SMALLINT");
        assert_eq!(_get_sql_type(&Value::I16(2)), "SMALLINT");
        assert_eq!(_get_sql_type(&Value::I32(3)), "INTEGER");
        assert_eq!(_get_sql_type(&Value::I64(4)), "BIGINT");
        assert_eq!(_get_sql_type(&Value::I128(5)), "NUMERIC(39,0)",);
        assert_eq!(_get_sql_type(&Value::ISize(6)), "INTEGER",);
        assert_eq!(_get_sql_type(&Value::U8(7)), "SMALLINT",);
        assert_eq!(_get_sql_type(&Value::U16(8)), "INTEGER",);
        assert_eq!(_get_sql_type(&Value::U32(9)), "BIGINT",);
        assert_eq!(_get_sql_type(&Value::U64(10)), "NUMERIC(20,0)",);
        assert_eq!(_get_sql_type(&Value::U128(11)), "NUMERIC(39,0)",);
        assert_eq!(_get_sql_type(&Value::USize(12)), "BIGINT",);
        assert_eq!(_get_sql_type(&Value::F32(1.0)), "REAL",);
        assert_eq!(_get_sql_type(&Value::F64(1.1)), "DOUBLE PRECISION",);
        assert_eq!(_get_sql_type(&Value::String("hello".to_string())), "TEXT",);
        assert_eq!(_get_sql_type(&Value::Blob(vec![])), "BYTEA",);
        assert_eq!(
            _get_sql_type(&Value::Date(Local::now().date_naive())),
            "DATE",
        );
    }

    #[test]
    fn test_to_sql_bool() {
        let wrapper = ValueWrapper(Value::Bool(true));
        let mut buf = BytesMut::new();
        let ty = &Type::BOOL;

        let result = wrapper.to_sql(ty, &mut buf).unwrap();
        assert!(matches!(result, IsNull::No));
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_to_sql_char() {
        let wrapper = ValueWrapper(Value::Char('x'));
        let mut buf = BytesMut::new();
        let result = wrapper.to_sql(&Type::TEXT, &mut buf).unwrap();
        assert!(matches!(result, IsNull::No));
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_to_sql_integers() {
        let values = vec![
            ValueWrapper(Value::I8(42)),
            ValueWrapper(Value::I16(42)),
            ValueWrapper(Value::I32(42)),
            ValueWrapper(Value::I64(42)),
            ValueWrapper(Value::I128(42)),
            ValueWrapper(Value::ISize(42)),
            ValueWrapper(Value::U8(42)),
            ValueWrapper(Value::U16(42)),
            ValueWrapper(Value::U32(42)),
            ValueWrapper(Value::U64(42)),
            ValueWrapper(Value::U128(42)),
            ValueWrapper(Value::USize(42)),
        ];

        for wrapper in values {
            let mut buf = BytesMut::new();
            let result = wrapper.to_sql(&Type::INT8, &mut buf).unwrap();
            assert!(matches!(result, IsNull::No));
            assert!(!buf.is_empty());
        }
    }

    #[test]
    fn test_to_sql_floats() {
        let values = vec![
            ValueWrapper(Value::F32(3.14)),
            ValueWrapper(Value::F64(2.718)),
        ];

        for wrapper in values {
            let mut buf = BytesMut::new();
            let result = wrapper.to_sql(&Type::FLOAT8, &mut buf).unwrap();
            assert!(matches!(result, IsNull::No));
            assert!(!buf.is_empty());
        }
    }

    #[test]
    fn test_to_sql_string() {
        let wrapper = ValueWrapper(Value::String("hello".to_string()));
        let mut buf = BytesMut::new();
        let ty = &Type::TEXT;

        let result = wrapper.to_sql(ty, &mut buf).unwrap();
        assert!(matches!(result, IsNull::No));
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_to_sql_blob() {
        let wrapper = ValueWrapper(Value::Blob(vec![1, 2, 3, 4]));
        let mut buf = BytesMut::new();
        let result = wrapper.to_sql(&Type::BYTEA, &mut buf).unwrap();
        assert!(matches!(result, IsNull::No));
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_to_sql_date() {
        use chrono::NaiveDate;
        let date = NaiveDate::from_ymd_opt(2023, 5, 1).unwrap();
        let wrapper = ValueWrapper(Value::Date(date));
        let mut buf = BytesMut::new();
        let ty = &Type::DATE;

        let result = wrapper.to_sql(ty, &mut buf).unwrap();
        assert!(matches!(result, IsNull::No));
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_to_sql_null_variant() {
        // Replace with a variant you don't handle in your match arm
        let wrapper = ValueWrapper(Value::None);
        let mut buf = BytesMut::new();
        let ty = &Type::TEXT;

        let result = wrapper.to_sql(ty, &mut buf).unwrap();
        assert!(matches!(result, IsNull::Yes));
        assert!(buf.is_empty());
    }
}
