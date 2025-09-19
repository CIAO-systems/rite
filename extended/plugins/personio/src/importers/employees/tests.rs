use std::collections::HashMap;

use model::import::handlers::ClosureRecordHandler;
use personio_rs::personnel::models::{
    AbsenceEntitlement, CostCenters, Department, DepartmentValue, DepartmentValueAttributes,
    Employee, EmployeeId, EmployeesResponse, EmployeesResponseAllOfData, HolidayCalendar,
    HolidayCalendarValue, HolidayCalendarValueAttributes, Office, OfficeValue,
    OfficeValueAttributes, ShortEmployee, ShortEmployeeAttributes, Supervisor, Team, TeamValue,
    TeamValueAttributes, WorkSchedule, WorkScheduleValue, WorkScheduleValueAttributes,
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

fn new_employee_response(id: &str) -> EmployeesResponseAllOfData {
    let mut e = EmployeesResponseAllOfData::new();
    let employee = new_employee(id);
    e.attributes = Some(Box::new(employee));
    e
}

fn new_employee(id: &str) -> Employee {
    let mut employee = Employee::new();
    let mut emp_id = EmployeeId::new();
    emp_id.value = Some(Some(json!(id)));
    employee.id = Some(Box::new(emp_id));
    employee
}

#[test]
fn test_handle_employee_response_success() {
    let importer = Employees::new();
    let response = EmployeesResponse {
        success: true,
        data: vec![new_employee_response("123"), new_employee_response("234")],
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

#[test]
fn test_create_record() {
    let importer = Employees::new();
    let mut employee = new_employee("123");
    employee.holiday_calendar = Some(
        HolidayCalendar {
            label: None,
            value: Some(
                HolidayCalendarValue {
                    r#type: None,
                    attributes: Some(HolidayCalendarValueAttributes::new().into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            r#type: None,
            universal_id: None,
            additional_properties: HashMap::new(),
        }
        .into(),
    );
    employee.supervisor = Some(
        Supervisor {
            label: None,
            value: Some(
                ShortEmployee {
                    r#type: None,
                    attributes: Some(ShortEmployeeAttributes::new().into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            additional_properties: HashMap::new(),
        }
        .into(),
    );

    employee.subcompany = Some(
        Office {
            label: None,
            value: Some(
                OfficeValue {
                    r#type: None,
                    attributes: Some(OfficeValueAttributes::new().into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            additional_properties: HashMap::new(),
            r#type: None,
            universal_id: None,
        }
        .into(),
    );

    employee.office = Some(
        Office {
            label: None,
            value: Some(
                OfficeValue {
                    r#type: None,
                    attributes: Some(OfficeValueAttributes::new().into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            additional_properties: HashMap::new(),
            r#type: None,
            universal_id: None,
        }
        .into(),
    );
    employee.department = Some(
        Department {
            label: None,
            value: Some(
                DepartmentValue {
                    r#type: None,
                    attributes: Some(DepartmentValueAttributes::new(1, "name".into()).into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            additional_properties: HashMap::new(),
            r#type: None,
            universal_id: None,
        }
        .into(),
    );
    employee.cost_centers = Some(
        CostCenters {
            label: None,
            value: Some(Vec::new()),
            additional_properties: HashMap::new(),
            r#type: None,
            universal_id: None,
        }
        .into(),
    );

    employee.work_schedule = Some(
        WorkSchedule {
            label: None,
            value: Some(
                WorkScheduleValue {
                    r#type: None,
                    attributes: Some(WorkScheduleValueAttributes::new().into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            additional_properties: HashMap::new(),
            r#type: None,
            universal_id: None,
        }
        .into(),
    );
    employee.absence_entitlement = Some(
        AbsenceEntitlement {
            label: None,
            value: Vec::new(),
            additional_properties: HashMap::new(),
        }
        .into(),
    );
    employee.team = Some(
        Team {
            label: None,
            value: Some(
                TeamValue {
                    r#type: None,
                    attributes: Some(TeamValueAttributes::new().into()),
                    additional_properties: HashMap::new(),
                }
                .into(),
            ),
            additional_properties: HashMap::new(),
            r#type: None,
            universal_id: None,
        }
        .into(),
    );

    employee.additional_properties = [
        ("prop1".into(), json!("value1")),
        ("prop2".into(), json!("value2")),
        ("prop3".into(), json!("value3")),
    ]
    .into_iter()
    .collect();

    let result = importer.create_record(employee.into());
    println!("{:?}", result);
    assert!(result.is_ok());
}
