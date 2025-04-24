use personio_rs::personnel::models::{AbsencePeriodsResponse, AttendancePeriodsResponse, EmployeesResponse};

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
