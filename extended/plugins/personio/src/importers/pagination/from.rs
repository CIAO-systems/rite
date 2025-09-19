use personio_rs::personnel::models::{
    AbsencePeriodsResponse, AttendancePeriodsResponse, EmployeesResponse,
};

use super::PageResult;

#[macro_export]
macro_rules! from_impl {
    ($classname:ident) => {
        /// Converts an [$classname] to a PageResult
        impl From<$classname> for PageResult<$classname> {
            fn from(value: $classname) -> Self {
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
    };
}

from_impl!(EmployeesResponse);
from_impl!(AttendancePeriodsResponse);
from_impl!(AbsencePeriodsResponse);

#[cfg(test)]
mod tests {
    use personio_rs::personnel::models::{EmployeesResponse, EmployeesResponseAllOfMetadata};

    use crate::importers::pagination::PageResult;

    #[test]
    fn test_macros() {
        let employee_response = EmployeesResponse::new(true, vec![]);
        let page_result: PageResult<EmployeesResponse> = employee_response.into();
        assert_eq!(page_result.total_pages, 0);

        let mut employee_response = EmployeesResponse::new(true, vec![]);
        employee_response.metadata = Some(EmployeesResponseAllOfMetadata::new(7342, 73, 42).into());
        let page_result: PageResult<EmployeesResponse> = employee_response.into();
        assert_eq!(page_result.total_pages, 73);
    }
}
