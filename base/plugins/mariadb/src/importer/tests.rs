use model::{record::Record, xml::config::Configuration};
use mysql::{Column, Value};
use rust_decimal::dec;

use crate::importer::{tests::mock::MockTableRow, *};

mod manual;
mod mock;

const THE_NAME_OF_THE_FIELD: &str = "the_name_of_the_field";
const TEST_STRING_VALUE: &str = "This are not the droids you are looking for";

#[test]
fn test_handle_decimal() {
    let values = vec![Value::from(dec!(42.73))];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();

    handle_decimal(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);

    assert_eq!(field.value(), model::value::Value::Decimal(dec!(42.73)));
}

fn std_field_asserts(fields: &mut Vec<Field>) -> &Field {
    assert_eq!(fields.len(), 1);
    let field = fields.get(0).unwrap();
    assert_eq!(field.name(), THE_NAME_OF_THE_FIELD);
    field
}

#[test]
fn test_handle_tiny_positive() {
    let values = vec![Value::from(73 as u8)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = consts::ColumnFlags::UNSIGNED_FLAG;
    handle_tiny(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::U8(73));
}

#[test]
fn test_handle_tiny_negative() {
    let values = vec![Value::from(-73 as i8)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = ColumnFlags::empty();
    handle_tiny(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::I8(-73));
}

#[test]
fn test_handle_short_positive() {
    let values = vec![Value::from(4273 as u16)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = consts::ColumnFlags::UNSIGNED_FLAG;
    handle_short(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::U16(4273));
}

#[test]
fn test_handle_short_negative() {
    let values = vec![Value::from(-4273 as i16)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = ColumnFlags::empty();
    handle_short(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::I16(-4273));
}

#[test]
fn test_handle_long_positive() {
    let values = vec![Value::from(4273 as u32)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = consts::ColumnFlags::UNSIGNED_FLAG;
    handle_long(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::U32(4273));
}

#[test]
fn test_handle_long_negative() {
    let values = vec![Value::from(-4273 as i32)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = ColumnFlags::empty();
    handle_long(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::I32(-4273));
}

#[test]
fn test_handle_longlong_positive() {
    let values = vec![Value::from(427342 as u64)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = consts::ColumnFlags::UNSIGNED_FLAG;
    handle_longlong(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::U64(427342));
}

#[test]
fn test_handle_longlong_negative() {
    let values = vec![Value::from(-4273 as i64)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    let col_flags = ColumnFlags::empty();
    handle_longlong(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD, &col_flags);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::I64(-4273));
}

#[test]
fn test_handle_float() {
    let values = vec![Value::from(42.73 as f32)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_float(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::F32(42.73));
}

#[test]
fn test_handle_double() {
    let values = vec![Value::from(4273.3724 as f64)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_double(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::F64(4273.3724));
}

#[test]
fn test_handle_null() {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_null(fields, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::None);
}

#[test]
fn test_handle_string() {
    let values = vec![Value::from(TEST_STRING_VALUE)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_string(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(
        field.value(),
        model::value::Value::String(TEST_STRING_VALUE.to_string())
    );
}

#[test]
fn test_handle_blob() {
    let values = vec![Value::from(TEST_STRING_VALUE.as_bytes())];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_blob(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(
        field.value(),
        model::value::Value::Blob(TEST_STRING_VALUE.as_bytes().to_vec())
    );
}

#[test]
fn test_handle_datetime() {
    let datetime = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(1991, 11, 24).unwrap(),
        NaiveTime::from_hms_opt(12, 13, 14).unwrap(),
    );
    let values = vec![Value::from(datetime)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_timestamp(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::DateTime(datetime));
}

#[test]
fn test_handle_date() {
    let date = NaiveDate::from_ymd_opt(1991, 11, 24).unwrap();
    let values = vec![Value::from(date)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_date(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::Date(date));
}

#[test]
fn test_handle_time() {
    let time = NaiveTime::from_hms_opt(12, 13, 14).unwrap();
    let values = vec![Value::from(time)];
    let mock_row = MockTableRow::from_values(values);

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    handle_time(&mock_row, fields, 0, THE_NAME_OF_THE_FIELD);
    let field = std_field_asserts(fields);
    assert_eq!(field.value(), model::value::Value::Time(time));
}

fn new_column(ct: ColumnType, name: &str) -> Column {
    Column::new(ct).with_name(name.as_bytes())
}

#[test]
fn test_handle_row() {
    let time = NaiveTime::from_hms_opt(12, 13, 14).unwrap();
    let date = NaiveDate::from_ymd_opt(1991, 11, 24).unwrap();
    let datetime = NaiveDateTime::new(date, time);

    let values = vec![
        Value::from(time),
        Value::from(date),
        Value::from(datetime),
        Value::from(TEST_STRING_VALUE.as_bytes()),
        Value::from(dec!(42.73)),
        Value::from(1 as u8),
        Value::from(2 as u16),
        Value::from(3 as u32),
        Value::from(4 as u64),
        Value::from(42.73 as f32),
        Value::from(73.42 as f64),
        Value::from(2025 as u32),
        Value::NULL,
        Value::from(time),
        Value::from(date),
        Value::from(datetime),
    ];
    let columns = vec![
        new_column(ColumnType::MYSQL_TYPE_TIME, "time"),
        new_column(ColumnType::MYSQL_TYPE_DATE, "date"),
        new_column(ColumnType::MYSQL_TYPE_DATETIME, "datetime"),
        new_column(ColumnType::MYSQL_TYPE_VARCHAR, "varchar"),
        new_column(ColumnType::MYSQL_TYPE_DECIMAL, "decimal"),
        new_column(ColumnType::MYSQL_TYPE_TINY, "tiny")
            .with_flags(consts::ColumnFlags::UNSIGNED_FLAG),
        new_column(ColumnType::MYSQL_TYPE_SHORT, "short")
            .with_flags(consts::ColumnFlags::UNSIGNED_FLAG),
        new_column(ColumnType::MYSQL_TYPE_LONG, "long")
            .with_flags(consts::ColumnFlags::UNSIGNED_FLAG),
        new_column(ColumnType::MYSQL_TYPE_LONGLONG, "longlong")
            .with_flags(consts::ColumnFlags::UNSIGNED_FLAG),
        new_column(ColumnType::MYSQL_TYPE_FLOAT, "float"),
        new_column(ColumnType::MYSQL_TYPE_DOUBLE, "double"),
        new_column(ColumnType::MYSQL_TYPE_YEAR, "year")
            .with_flags(consts::ColumnFlags::UNSIGNED_FLAG),
        new_column(ColumnType::MYSQL_TYPE_NULL, "null"),
        new_column(ColumnType::MYSQL_TYPE_TIME2, "time2"),
        new_column(ColumnType::MYSQL_TYPE_NEWDATE, "newdate"),
        new_column(ColumnType::MYSQL_TYPE_DATETIME2, "datetime2"),
    ];

    let mock_row = MockTableRow::new(values, columns);

    let result = handle_row(&mock_row);
    println!("{:?}", result);
    assert!(result.is_ok());

    let record = result.ok().unwrap();
    assert_eq!(
        record.field_by_name("time").unwrap().value(),
        model::value::Value::Time(time)
    );
    assert_eq!(
        record.field_by_name("date").unwrap().value(),
        model::value::Value::Date(date)
    );
    assert_eq!(
        record.field_by_name("datetime").unwrap().value(),
        model::value::Value::DateTime(datetime)
    );
    assert_eq!(
        record.field_by_name("time2").unwrap().value(),
        model::value::Value::Time(time)
    );
    assert_eq!(
        record.field_by_name("newdate").unwrap().value(),
        model::value::Value::Date(date)
    );
    assert_eq!(
        record.field_by_name("datetime2").unwrap().value(),
        model::value::Value::DateTime(datetime)
    );
    assert_eq!(
        record.field_by_name("varchar").unwrap().value(),
        model::value::Value::String(TEST_STRING_VALUE.into())
    );

    assert_eq!(
        record.field_by_name("decimal").unwrap().value(),
        model::value::Value::Decimal(dec!(42.73))
    );

    assert_eq!(
        record.field_by_name("tiny").unwrap().value(),
        model::value::Value::U8(1)
    );
    assert_eq!(
        record.field_by_name("short").unwrap().value(),
        model::value::Value::U16(2)
    );
    assert_eq!(
        record.field_by_name("long").unwrap().value(),
        model::value::Value::U32(3)
    );
    assert_eq!(
        record.field_by_name("longlong").unwrap().value(),
        model::value::Value::U64(4)
    );
    assert_eq!(
        record.field_by_name("float").unwrap().value(),
        model::value::Value::F32(42.73)
    );
    assert_eq!(
        record.field_by_name("double").unwrap().value(),
        model::value::Value::F64(73.42)
    );
    assert_eq!(
        record.field_by_name("year").unwrap().value(),
        model::value::Value::U32(2025)
    );
    assert_eq!(
        record.field_by_name("null").unwrap().value(),
        model::value::Value::None
    );
}

#[test]
fn test_handle_row_with_unknown() {
    let values = vec![
        Value::from(1),
        Value::from(2.0),
        Value::from(vec![]),
        Value::from(TEST_STRING_VALUE.as_bytes()),
    ];
    let columns = vec![
        Column::new(ColumnType::MYSQL_TYPE_TINY).with_name("tiny".as_bytes()),
        Column::new(ColumnType::MYSQL_TYPE_FLOAT).with_name("float".as_bytes()),
        Column::new(ColumnType::MYSQL_TYPE_BLOB).with_name("blob".as_bytes()),
        Column::new(ColumnType::MYSQL_TYPE_VARCHAR).with_name("varchar".as_bytes()),
        Column::new(ColumnType::MYSQL_TYPE_UNKNOWN).with_name("unknown".as_bytes()),
    ];

    let mock_row = MockTableRow::new(values, columns);

    let result = handle_row(&mock_row);
    println!("{:?}", result);
    assert!(
        result.is_err_and(|e| e.to_string() == "Unsupported type: MYSQL_TYPE_UNKNOWN for unknown")
    );
}

#[test]
fn test_init() {
    let mut importer = MariaDBImporter::new();
    let xml_file = "../../data/test/mariadb/mariadb-import-config.xml";
    let config = Configuration::with_xml(xml_file);
    let result = importer.init(Some(config));
    assert!(result.is_ok());
    assert!(importer.mariadb.is_some());
    let mariadb = importer.mariadb.unwrap();
    assert_eq!(mariadb.connection.host, "localhost");
    assert_eq!(mariadb.connection.port, 3306);
    assert_eq!(mariadb.connection.database, "mariadb");
    assert_eq!(mariadb.connection.user, "user");
    assert_eq!(mariadb.connection.password, "topsecret");
    assert_eq!(mariadb.sql, "select * from customers");
}
