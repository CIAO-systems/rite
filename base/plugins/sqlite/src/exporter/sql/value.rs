use rusqlite::{ToSql, types::ToSqlOutput};

#[derive(Clone, Debug)]
pub struct ValueWrapper(pub model::value::Value);

impl ToSql for ValueWrapper {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let r = match &self.0 {
            model::value::Value::Bool(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer(if *v { 1 } else { 0 }))
            }
            model::value::Value::Char(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Text(format!("{v}")))
            }
            model::value::Value::I8(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::I16(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::I32(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::I64(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::U8(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::U16(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::U32(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Integer((*v).into()))
            }
            model::value::Value::F32(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Real((*v).into()))
            }
            model::value::Value::F64(v) => {
                ToSqlOutput::Owned(rusqlite::types::Value::Real((*v).into()))
            }
            model::value::Value::String(v) => ToSqlOutput::Borrowed(v.as_str().into()),
            model::value::Value::Blob(items) => ToSqlOutput::Borrowed(items.as_slice().into()),
            _ => ToSqlOutput::Owned(rusqlite::types::Value::Null),
        };

        Ok(r)
    }
}

impl From<ValueWrapper> for rusqlite::types::Value {
    fn from(value: ValueWrapper) -> Self {
        match value.0 {
            model::value::Value::Bool(v) => rusqlite::types::Value::Integer(if v { 1 } else { 0 }),
            model::value::Value::Char(v) => rusqlite::types::Value::Text(format!("{v}")),
            model::value::Value::I8(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::I16(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::I32(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::I64(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::U8(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::U16(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::U32(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::F32(v) => rusqlite::types::Value::Real(v.into()),
            model::value::Value::F64(v) => rusqlite::types::Value::Real(v),
            model::value::Value::String(v) => rusqlite::types::Value::Text(v),
            model::value::Value::Blob(items) => rusqlite::types::Value::Blob(items),
            /*
            model::value::Value::U64(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::U128(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::USize(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::I128(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::ISize(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::Date(naive_date) => todo!(),
            model::value::Value::DateTime(naive_date_time) => todo!(),
            model::value::Value::Time(naive_time) => todo!(),
            model::value::Value::Collection(values) => todo!(),
            model::value::Value::Record(record) => todo!(),
            model::value::Value::Decimal(decimal) => todo!(),
            */
            _ => rusqlite::types::Value::Null,
        }
    }
}
