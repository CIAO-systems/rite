use model::record::Record;
use personio_rs::personnel::models::{
    Attendance, AttendancePeriodsResponseAllOfData, ShortEmployee, ShortEmployeeAttributes,
    ShortEmployeeAttributesId,
};
use serde_json::json;

#[derive(Debug)]
struct Inner {
    value: String,
}

#[derive(Debug)]
struct Field {
    value: Option<Option<Inner>>,
}

#[derive(Debug)]
struct Attr {
    field: Option<Field>,
}

fn create_struct() -> Attr {
    Attr {
        field: Some(Field {
            value: Some(Some(Inner {
                value: "hello".into(),
            })),
        }),
    }
}

#[test]
fn test_macro_unpack_attribute() {
    // Arrange
    let attr = create_struct();

    // Act
    let result = unpack_attribute!(attr, field, value);

    // Assert
    assert_eq!(result.unwrap().value, "hello");
}

#[test]
fn test_macro_get_value() {
    // Arrange
    let attr = create_struct();

    // Act
    let result = get_value!(attr, field);

    // Assert
    assert_eq!(result.unwrap().value, "hello");
}

#[test]
fn test_macro_add_field_option() {
    // Arrange
    let attendance = Attendance::new();
    let mut attributes = AttendancePeriodsResponseAllOfData::new(1, None, attendance).attributes;
    attributes.end_time = Some(Some("the end time".into()));
    println!("{:?}", attributes);

    let mut record = Record::new();

    // Act
    add_field_option!(record, attributes, end_time);

    // Assert
    println!("{:?}", record);
    assert!(
        record
            .field_by_name("end_time")
            .is_some_and(|f| f.value().to_string() == "the end time")
    );
}

#[test]
fn test_macro_add_field_boxed() {
    // Arrange
    let mut employee = Box::new(ShortEmployee::new());
    let mut empattr = ShortEmployeeAttributes::new();
    let mut id = ShortEmployeeAttributesId::new();
    id.value = Some(Some(json!("value")));
    empattr.id = Some(id.into());
    employee.attributes = Some(empattr.into());

    let attributes = employee.attributes.unwrap();
    let mut record = Record::new();

    // Act
    add_field_boxed!(record, attributes, id);

    // Assert
    println!("{:?}", record);
    assert!(
        record
            .field_by_name("id")
            .is_some_and(|f| f.value().to_string() == "value")
    );
}
