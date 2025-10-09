use model::record::Record;
use mysql::Value;
use rust_decimal::dec;

use crate::importer::{tests::mock::MockTableRow, *};

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

#[test]
fn test_handle_row() {
    let values = vec![];
    let mock_row = MockTableRow::from_values(values);

    let result = handle_row(&mock_row);
    assert!(result.is_ok())
}
