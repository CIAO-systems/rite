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

#[cfg(test)]
mod tests {
    use rite_sql::DatabaseFlavor;

    use crate::exporter::sql::{MariaDBFlavor, value};

    #[test]
    fn test_placeholder() {
        let placeholder = MariaDBFlavor::placeholder(0);
        assert_eq!(placeholder, "?");
    }

    #[test]
    fn test_wrap_value() {
        let wrapped_value = MariaDBFlavor::wrap_value(model::value::Value::I32(8472));
        assert!(matches!(
            wrapped_value,
            value::ValueWrapper(model::value::Value::I32(8472))
        ));
    }
}
