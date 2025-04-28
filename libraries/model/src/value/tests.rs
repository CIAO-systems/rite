use chrono::{Local, NaiveDate};

use crate::{field::add_field, record::Record};

use super::Value;

#[test]
fn test_print_bool() {
    for b in [true, false] {
        let v = Value::Bool(b);
        let s = format!("{v}");

        assert_eq!(b.to_string(), s);
    }
}

#[test]
fn test_print_char() {
    for c in ['a', 'b', 'c'] {
        let v = Value::Char(c);
        let s = format!("{v}");

        assert_eq!(c.to_string(), s);
    }
}

#[test]
fn test_print_i8() {
    let v = Value::I8(7);
    let s = format!("{v}");

    assert_eq!("7", s);
}

#[test]
fn test_print_i16() {
    let v = Value::I16(73);
    let s = format!("{v}");

    assert_eq!("73", s);
}

#[test]
fn test_print_i32() {
    let v = Value::I32(73000);
    let s = format!("{v}");

    assert_eq!("73000", s);
}

#[test]
fn test_print_i64() {
    let i: i64 = -3647181746182187;
    let v = Value::I64(i);
    let s = format!("{v}");

    assert_eq!("-3647181746182187", s);
}

#[test]
fn test_print_i128() {
    let i: i128 = -5734892329309810874233647181746182187;
    let v = Value::I128(i);
    let s = format!("{v}");

    assert_eq!("-5734892329309810874233647181746182187", s);
}

#[test]
fn test_print_isize() {
    let i: isize = -4233647181746182187;
    let v = Value::ISize(i);
    let s = format!("{v}");

    assert_eq!("-4233647181746182187", s);
}

#[test]
fn test_print_u8() {
    let v = Value::U8(7);
    let s = format!("{v}");

    assert_eq!("7", s);
}

#[test]
fn test_print_u16() {
    let v = Value::U16(73);
    let s = format!("{v}");

    assert_eq!("73", s);
}

#[test]
fn test_print_u32() {
    let v = Value::U32(73000);
    let s = format!("{v}");

    assert_eq!("73000", s);
}

#[test]
fn test_print_u64() {
    let i: u64 = 3647181746182187;
    let v = Value::U64(i);
    let s = format!("{v}");

    assert_eq!("3647181746182187", s);
}

#[test]
fn test_print_u128() {
    let i: u128 = 5724892329309810874233647181746182187;
    let v = Value::U128(i);
    let s = format!("{v}");

    assert_eq!("5724892329309810874233647181746182187", s);
}

#[test]
fn test_print_usize() {
    let i: usize = 6233647181746182187;
    let v = Value::USize(i);
    let s = format!("{v}");

    assert_eq!("6233647181746182187", s);
}

#[test]
fn test_print_f32() {
    let f: f32 = 73.42;
    let v = Value::F32(f);
    let s = format!("{v}");

    assert_eq!(format!("{:.}", f), s);
}

#[test]
fn test_print_f64() {
    let f: f64 = 73.42 * 42.73;
    let v = Value::F64(f);
    let s = format!("{v}");

    assert_eq!(format!("{:.}", f), s);
}

#[test]
fn test_print_string() {
    let exp_str = "This are not the droids you're looking for";
    let v = Value::String(exp_str.to_string());
    let s = format!("{v}");

    assert_eq!(exp_str, s);
}

#[test]
fn test_print_blob() {
    let exp_blob: &[u8] = b"This are not the droids you're looking for";
    let v = Value::Blob(exp_blob.to_vec());
    let s = format!("{v}");

    assert_eq!("[54, 68, 69, 73, 20, 61, 72, 65, 20, 6e, 6f, 74, 20, 74, 68, 65, 20, 64, 72, 6f, 69, 64, 73, 20, 79, 6f, 75, 27, 72, 65, 20, 6c, 6f, 6f, 6b, 69, 6e, 67, 20, 66, 6f, 72]", s);
}

#[test]
fn test_print_date() {
    let exp: NaiveDate = Local::now().date_naive();
    let v = Value::Date(exp);
    let s = format!("{v}");

    assert_eq!(exp.to_string(), s);
}

#[test]
fn test_print_collection() {
    let exp: NaiveDate = Local::now().date_naive();
    let v1 = Value::Date(exp);
    let v2 = Value::String("This is a string!".to_string());
    let v3 = Value::I32(42);

    let v = Value::Collection(vec![v1, v2, v3]);
    let s = format!("{v}");

    assert_eq!(format!("[{}, This is a string!, 42]", exp.to_string()), s);
}

#[test]
fn test_print_record() {
    let mut record = Record::new();
    let date: NaiveDate = Local::now().date_naive();
    let v1 = Value::Date(date);
    let v2 = Value::String("This is a string!".to_string());
    let v3 = Value::I32(42);

    add_field(record.fields_as_mut(), "date", v1);
    add_field(record.fields_as_mut(), "string", v2);
    add_field(record.fields_as_mut(), "i32", v3);
    
    let v = Value::Record(record);
    let s = format!("{v}");

    assert_eq!(format!("{{date={}, string=This is a string!, i32=42}}", date.to_string()), s);
}

#[test]
fn test_print_none() {
    let none = Value::None;
    let s = format!("{none}");

    assert_eq!("<None>", s);
}
