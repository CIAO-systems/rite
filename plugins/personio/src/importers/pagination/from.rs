use personio_rs::personnel::models::{AttendancePeriodsResponse, EmployeesResponse};

use super::PageResult;

/// Converts an [EmployeesResponse] to a PageResult
impl From<EmployeesResponse> for PageResult<EmployeesResponse> {
    fn from(value: EmployeesResponse) -> Self {
        let total_pages: i32 = if let Some(ref meta_data) = value.metadata {
            meta_data.total_pages
        } else {
            0
        };

        PageResult {
            data: value,
            total_pages,
        }
    }
}

/// Converts an [AttendancePeriodsResponse] to a PageResult
impl From<AttendancePeriodsResponse> for PageResult<AttendancePeriodsResponse> {
    fn from(value: AttendancePeriodsResponse) -> Self {
        let total_pages: i32 = if let Some(ref meta_data) = value.metadata {
            meta_data.total_pages
        } else {
            0
        };

        PageResult {
            data: value,
            total_pages,
        }
    }
}
