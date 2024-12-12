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
            Value::None => Ok(IsNull::Yes),
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
        Value::None => "TEXT", // Default for nullable column
    }
}
