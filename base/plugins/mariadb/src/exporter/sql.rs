use rite_sql::DatabaseFlavor;

mod value;

pub struct MariaDBFlavor;

impl DatabaseFlavor for MariaDBFlavor {
    type ValueWrapper = value::ValueWrapper;

    fn placeholder(_index: usize) -> String {
        format!("?")
    }

    fn wrap_value(value: model::value::Value) -> Self::ValueWrapper {
        value::ValueWrapper(value)
    }
}

pub mod error_code {
    /// Duplicate entry for key (unique or primary key violation)
    pub const ER_DUP_ENTRY: u16 = 1062;
}
