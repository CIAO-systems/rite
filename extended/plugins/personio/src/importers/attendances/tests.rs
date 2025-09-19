use std::collections::HashMap;

use model::import::handlers::ClosureRecordHandler;
use personio_rs::personnel::models::AttendancePeriodsResponse;

use crate::importers::attendances::Attendances;

#[test]
fn test_handle_attendance_response() {
    let attendances = Attendances::new();
    let mut handler = ClosureRecordHandler::new(|r| println!("{:?}", r));
    let page = AttendancePeriodsResponse {
        success: true,
        data: Vec::new(),
        metadata: None,
        offset: None,
        limit: None,
        additional_properties: HashMap::new(),
    };
    let result = attendances.handle_attendance_response(&mut handler, page);
    println!("{:?}", result);
    assert!(result.is_ok());
}
