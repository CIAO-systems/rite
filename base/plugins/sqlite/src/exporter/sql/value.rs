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
            model::value::Value::Date(naive_date) => {
                ToSqlOutput::Owned(naive_date.format("%Y-%m-%d").to_string().into())
            }
            model::value::Value::DateTime(naive_date_time) => ToSqlOutput::Owned(
                naive_date_time
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .into(),
            ),
            model::value::Value::Time(naive_time) => {
                ToSqlOutput::Owned(naive_time.format("%H:%M:%S").to_string().into())
            }

            _ => ToSqlOutput::Owned(rusqlite::types::Value::Null),
        };

        Ok(r)
    }
}

impl From<ValueWrapper> for rusqlite::types::Value {
    fn from(value: ValueWrapper) -> Self {
        match value.0 {
            model::value::Value::Bool(v) => v.into(),
            model::value::Value::Char(v) => rusqlite::types::Value::Text(format!("{v}")),
            model::value::Value::I8(v) => v.into(),
            model::value::Value::I16(v) => v.into(),
            model::value::Value::I32(v) => v.into(),
            model::value::Value::I64(v) => v.into(),
            model::value::Value::U8(v) => v.into(),
            model::value::Value::U16(v) => v.into(),
            model::value::Value::U32(v) => v.into(),
            model::value::Value::F32(v) => v.into(),
            model::value::Value::F64(v) => v.into(),
            model::value::Value::String(v) => v.into(),
            model::value::Value::Blob(v) => v.into(),
            model::value::Value::Date(naive_date) => {
                rusqlite::types::Value::Text(naive_date.format("%Y-%m-%d").to_string())
            }
            model::value::Value::DateTime(naive_date_time) => rusqlite::types::Value::Text(
                naive_date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
            model::value::Value::Time(naive_time) => {
                rusqlite::types::Value::Text(naive_time.format("%H:%M:%S").to_string())
            }
            /*
            model::value::Value::U64(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::U128(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::USize(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::I128(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::ISize(v) => rusqlite::types::Value::Integer(v.into()),
            model::value::Value::Collection(values) => todo!(),
            model::value::Value::Record(record) => todo!(),
            model::value::Value::Decimal(decimal) => todo!(),
            */
            _ => rusqlite::types::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use rusqlite::{ToSql, types::ToSqlOutput};

    use crate::exporter::sql::value::ValueWrapper;

    #[test]
    fn test_from_and_to_sql_bool() {
        let value = model::value::Value::Bool(true);
        let wrapper = ValueWrapper(value);
        let sql_value = wrapper.to_sql();
        println!("{:?}", sql_value);
        assert_eq!(
            sql_value.unwrap(),
            ToSqlOutput::Owned(rusqlite::types::Value::Integer(1))
        );

        let from_value: rusqlite::types::Value = wrapper.into();
        println!("{:?}", from_value);
        assert_eq!(from_value, rusqlite::types::Value::Integer(1));
    }

    #[test]
    fn test_from_and_to_sql_char() {
        let value = model::value::Value::Char('A');
        let wrapper = ValueWrapper(value);
        let sql_value = wrapper.to_sql();
        println!("{:?}", sql_value);
        assert_eq!(
            sql_value.unwrap(),
            ToSqlOutput::Owned(rusqlite::types::Value::Text("A".into()))
        );

        let from_value: rusqlite::types::Value = wrapper.into();
        println!("{:?}", from_value);
        assert_eq!(from_value, rusqlite::types::Value::Text("A".into()));
    }

    #[test]
    fn test_from_and_to_sql_none() {
        let value = model::value::Value::None;
        let wrapper = ValueWrapper(value);
        let sql_value = wrapper.to_sql();
        println!("{:?}", sql_value);
        assert_eq!(
            sql_value.unwrap(),
            ToSqlOutput::Owned(rusqlite::types::Value::Null)
        );

        let from_value: rusqlite::types::Value = wrapper.into();
        println!("{:?}", from_value);
        assert_eq!(from_value, rusqlite::types::Value::Null);
    }

    #[test]
    fn test_from_and_to_sql_int() {
        let int_values = [
            model::value::Value::I8(1),
            model::value::Value::I16(2),
            model::value::Value::I32(3),
            model::value::Value::I64(4),
            model::value::Value::U8(5),
            model::value::Value::U16(6),
            model::value::Value::U32(7),
        ];

        for (i, value) in int_values.iter().enumerate() {
            let expected: i64 = i as i64 + 1;
            let wrapper = ValueWrapper(value.to_owned());
            let sql_value = wrapper.to_sql();
            println!("{:?}", sql_value);
            assert_eq!(
                sql_value.unwrap(),
                ToSqlOutput::Owned(rusqlite::types::Value::Integer(expected))
            );

            let from_value: rusqlite::types::Value = wrapper.into();
            println!("{:?}", from_value);
            assert_eq!(from_value, rusqlite::types::Value::Integer(expected));
        }
    }

    #[test]
    fn test_from_and_to_sql_float() {
        let float_values = [model::value::Value::F32(1.0), model::value::Value::F64(2.0)];

        for (i, value) in float_values.iter().enumerate() {
            let expected: f64 = i as f64 + 1.0;
            let wrapper = ValueWrapper(value.to_owned());
            let sql_value = wrapper.to_sql();
            println!("{:?}", sql_value);
            assert_eq!(
                sql_value.unwrap(),
                ToSqlOutput::Owned(rusqlite::types::Value::Real(expected))
            );

            let from_value: rusqlite::types::Value = wrapper.into();
            println!("{:?}", from_value);
            assert_eq!(from_value, rusqlite::types::Value::Real(expected));
        }
    }

    #[test]
    fn test_from_and_to_sql_string() {
        const TEST_STRING: &str = "This are not the droids you are looking for";
        let value = model::value::Value::String(TEST_STRING.into());
        let wrapper = ValueWrapper(value);
        let sql_value = wrapper.to_sql();
        println!("{:?}", sql_value);
        assert_eq!(
            sql_value.unwrap(),
            ToSqlOutput::Borrowed(TEST_STRING.into())
        );

        let from_value: rusqlite::types::Value = wrapper.into();
        println!("{:?}", from_value);
        assert_eq!(from_value, rusqlite::types::Value::Text(TEST_STRING.into()));
    }

    #[test]
    fn test_from_and_to_sql_blob() {
        const TEST_DATA: [u8; 4] = [1, 2, 3, 4];

        let value = model::value::Value::Blob(TEST_DATA.into());
        let wrapper = ValueWrapper(value);
        let sql_value = wrapper.to_sql();
        println!("{:?}", sql_value);
        assert_eq!(
            sql_value.unwrap(),
            ToSqlOutput::Borrowed(TEST_DATA.as_slice().into())
        );

        let from_value: rusqlite::types::Value = wrapper.into();
        println!("{:?}", from_value);
        assert_eq!(
            from_value,
            rusqlite::types::Value::Blob(TEST_DATA.as_slice().into())
        );
    }

    #[test]
    fn test_from_and_to_sql_chrono() {
        let date = NaiveDate::from_ymd_opt(2025, 10, 21).expect("Invalid date");
        let time = NaiveTime::from_hms_opt(14, 30, 45).expect("Invalid time");
        let expected = [
            model::value::Value::Date(date),
            model::value::Value::Time(time),
            model::value::Value::DateTime(NaiveDateTime::new(date, time)),
        ];

        for (i, value) in expected.iter().enumerate() {
            let wrapper = ValueWrapper(value.to_owned());
            let sql_value = wrapper.to_sql();
            println!("{:?}", sql_value);
            assert_eq!(
                sql_value.unwrap(),
                ToSqlOutput::Owned(rusqlite::types::Value::Text(expected[i].to_string()))
            );

            let from_value: rusqlite::types::Value = wrapper.into();
            println!("{:?}", from_value);
            assert_eq!(
                from_value,
                rusqlite::types::Value::Text(expected[i].to_string())
            );
        }
    }
}
