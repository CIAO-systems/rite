use rite_sql::DatabaseFlavor;

mod value;

pub struct PostgresFlavor;

impl DatabaseFlavor for PostgresFlavor {
    type ValueWrapper = value::ValueWrapper;

    fn placeholder(index: usize) -> String {
        format!("${index}")
    }

    fn wrap_value(value: model::value::Value) -> Self::ValueWrapper {
        value::ValueWrapper(value)
    }
}


#[cfg(test)]
mod tests;
