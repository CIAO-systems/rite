use model::{record::Record, value::Value};
use std::{collections::HashSet, error::Error};

pub trait DatabaseFlavor {
    type ValueWrapper;

    /// Returns the parameter placeholder
    fn placeholder(index: usize) -> String;

    /// Wraps a raw model value in the database specific ValueWrapper
    fn wrap_value(value: Value) -> Self::ValueWrapper;
}

/// A generic parameterized query
pub struct ParameterizedQuery<T> {
    pub sql: String,
    pub params: Vec<T>,
}

pub fn generate_insert_statement<F: DatabaseFlavor>(
    table_name: &str,
    record: &Record,
) -> Result<ParameterizedQuery<F::ValueWrapper>, Box<dyn Error>> {
    let mut column_names = Vec::new();
    let mut param_placeholders = Vec::new();
    let mut params = Vec::new();

    let mut index = 1;
    for field in record.fields() {
        column_names.push(field.name());
        param_placeholders.push(F::placeholder(index));
        params.push(F::wrap_value(field.value()));
        index += 1;
    }

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table_name,
        column_names.join(", "),
        param_placeholders.join(", ")
    );

    Ok(ParameterizedQuery { sql, params })
}

pub fn generate_update_statement<F: DatabaseFlavor>(
    table_name: &str,
    record: &Record,
    unique_fields: &HashSet<&str>,
) -> Result<ParameterizedQuery<F::ValueWrapper>, Box<dyn Error>> {
    // Separate storage to maintain order preference (SET before WHERE)
    let mut set_data = Vec::new();
    let mut where_data = Vec::new();

    for field in record.fields() {
        let name = field.name();
        let value = F::wrap_value(field.value());

        if unique_fields.contains(name) {
            // WHERE clause field
            where_data.push((name, value));
        } else {
            // SET clause field
            set_data.push((name, value));
        }
    }

    if set_data.is_empty() {
        return Err("No non-unique fields to update".into());
    }

    if where_data.is_empty() {
        return Err("No unique fields specified for WHERE clause (required for update)".into());
    }

    let mut set_clauses = Vec::new();
    let mut where_clauses = Vec::new();
    let mut params = Vec::new(); // Final, contiguous parameter list
    let mut index = 1; // Start index

    // Create SET clause first
    for (name, value) in set_data.into_iter() {
        set_clauses.push(format!("{name} = {}", F::placeholder(index)));
        params.push(value);
        index += 1;
    }

    // Create WHERE clause second
    for (name, value) in where_data.into_iter() {
        where_clauses.push(format!("{name} = {}", F::placeholder(index)));
        params.push(value);
        index += 1;
    }

    //Construct SQL
    let sql = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name,
        set_clauses.join(", "),
        where_clauses.join(" AND ")
    );

    Ok(ParameterizedQuery { sql, params })
}

#[cfg(test)]
mod tests;
