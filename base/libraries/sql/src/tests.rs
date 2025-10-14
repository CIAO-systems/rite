use model::{
    field::{Field, add_field},
    record::Record,
    value::Value,
};

use crate::{DatabaseFlavor, generate_insert_statement, generate_update_statement};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TestValueWrapper(Value);

pub struct TestDatabaseFlavor;

impl DatabaseFlavor for TestDatabaseFlavor {
    type ValueWrapper = TestValueWrapper;

    fn placeholder(index: usize) -> String {
        format!("<{index}>")
    }

    fn wrap_value(value: Value) -> Self::ValueWrapper {
        TestValueWrapper(value)
    }
}

#[test]
fn test_generate_insert() {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "bool", Value::Bool(true));
    add_field(fields, "int", Value::I32(73));

    let query = generate_insert_statement::<TestDatabaseFlavor>("table_name", &record);
    assert!(query.is_ok());
    let query = query.unwrap();
    assert_eq!(
        query.sql,
        "INSERT INTO table_name (bool, int) VALUES (<1>, <2>);"
    );
    assert_eq!(query.params.len(), 2);
}

#[test]
fn test_generate_update() {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    add_field(fields, "id", Value::I128(0));
    add_field(fields, "bool", Value::Bool(true));
    add_field(fields, "int", Value::I32(73));

    let unique_fields = ["id"].iter().cloned().collect();
    let query =
        generate_update_statement::<TestDatabaseFlavor>("table_name", &record, &unique_fields);
    assert!(query.is_ok());
    let query = query.unwrap();
    assert_eq!(
        query.sql,
        "UPDATE table_name SET bool = <1>, int = <2> WHERE id = <3>;"
    );
    assert_eq!(query.params.len(), 3);
}

pub struct AnotherTestDatabaseFlavor;

impl DatabaseFlavor for AnotherTestDatabaseFlavor {
    type ValueWrapper = TestValueWrapper;

    fn placeholder(index: usize) -> String {
        format!("${index}")
    }

    fn wrap_value(value: Value) -> Self::ValueWrapper {
        TestValueWrapper(value)
    }
}

#[test]
fn test_generate_insert_statement() {
    let mut record = Record::new();
    let fields = record.fields_as_mut();
    fields.push(Field::new_value("index", Value::I32(0)));
    fields.push(Field::new_value(
        "name",
        Value::String("Some name".to_string()),
    ));

    if let Ok(statement) =
        generate_insert_statement::<AnotherTestDatabaseFlavor>("tablename", &record)
    {
        assert_eq!(
            "INSERT INTO tablename (index, name) VALUES ($1, $2);",
            statement.sql
        );
    }
}

#[test]
fn test_generate_update_statement() {
    let expected = [
        Value::String("Some name".to_string()),
        Value::I32(0),
        Value::String("user@company".to_string()),
    ];

    let mut record = Record::new();
    let fields = record.fields_as_mut();
    fields.push(Field::new_value("name", expected[0].clone()));
    fields.push(Field::new_value("index", expected[1].clone()));
    fields.push(Field::new_value("email", expected[2].clone()));

    let unique_fields = ["index", "email"].iter().cloned().collect();
    if let Ok(statement) =
        generate_update_statement::<AnotherTestDatabaseFlavor>("tablename", &record, &unique_fields)
    {
        assert_eq!(
            "UPDATE tablename SET name = $1 WHERE index = $2 AND email = $3;",
            statement.sql
        );

        println!("params={:?}", statement.params);
        println!("expected={:?}", expected);
        assert_eq!(3, statement.params.len());
        for (i, value) in expected.iter().enumerate() {
            println!(
                "i={:?}, value={:?}, param[i]={:?}",
                i, value, statement.params[i].0
            );
            assert_eq!(*value, statement.params[i].0);
        }
    }
}
