use std::collections::HashMap;

use model::import::handlers::ClosureRecordHandler;
use personio_rs::personnel::models::{
    Employee, EmployeeId, EmployeesResponse, EmployeesResponseAllOfData,
};
use serde_json::json;

use crate::importers::employees::{Employees, EmployeesFilter};

#[test]
fn test_filter_set_attribute() {
    let mut filter = EmployeesFilter::new();
    filter.set_attributes("a1,a2,a3".into());
    assert!(filter.attributes.is_some());
    let attr = filter.attributes.unwrap();
    assert_eq!(attr.len(), 3);
}

#[test]
fn test_employees_is_flag_set() {
    let employees = Employees::new();

    assert!(!employees.is_flag_set("flag"));
}

// FIXME this will always try to authenticate with Personio and it can't be mocked currently
// #[test]
// fn test_employees_init() {
//     let mut importer = Employees::new();
//     let mut config = Configuration::new();
//     config.insert_str("client_id", "dummy-client");
//     config.insert_str("client_secret", "dummy-secret");
//     let result = importer.init(Some(config));
//     println!("{:?}", result);
//     assert!(result.is_ok());
// }

fn new_employee(id: &str) -> EmployeesResponseAllOfData {
    let mut e = EmployeesResponseAllOfData::new();
    let mut employee = Employee::new();
    let mut emp_id = EmployeeId::new();
    emp_id.value = Some(Some(json!(id)));
    employee.id = Some(Box::new(emp_id));
    e.attributes = Some(Box::new(employee));
    e
}

#[test]
fn test_handle_employee_response_success() {
    let importer = Employees::new();
    let response = EmployeesResponse {
        success: true,
        data: vec![new_employee("123"), new_employee("234")],
        metadata: None,
        offset: None,
        limit: None,
        additional_properties: HashMap::new(),
    };
    let mut ids = Vec::new();
    let mut handler = ClosureRecordHandler::new(|r| {
        println!("{:?}", r);
        ids.push(r.field_by_name("id").unwrap().value().clone());
    });

    let result = importer.handle_employee_response(&mut handler, response);
    println!("{:?}", result);
    assert!(result.is_ok());

    assert!(!ids.is_empty());
}

#[test]
fn test_handle_employee_response_failure() {
    let importer = Employees::new();
    let response = EmployeesResponse {
        success: false,
        data: vec![],
        metadata: None,
        offset: None,
        limit: None,
        additional_properties: HashMap::new(),
    };
    let mut ids = Vec::new();
    let mut handler = ClosureRecordHandler::new(|r| {
        println!("{:?}", r);
        ids.push(r.field_by_name("id").unwrap().value().clone());
    });

    let result = importer.handle_employee_response(&mut handler, response);
    println!("{:?}", result);
    assert!(result.is_err());

    assert!(ids.is_empty());
}
