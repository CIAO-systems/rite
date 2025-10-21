use rite_sql::DatabaseFlavor;

use crate::exporter::sql::value::ValueWrapper;

mod value;

pub struct SQLiteFlavor;

impl DatabaseFlavor for SQLiteFlavor {
    type ValueWrapper = value::ValueWrapper;

    fn placeholder(index: usize) -> String {
        format!("?{}", index).into()
    }

    fn wrap_value(value: model::value::Value) -> Self::ValueWrapper {
        ValueWrapper(value)
    }
}
