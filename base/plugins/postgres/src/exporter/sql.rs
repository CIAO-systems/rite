use model::{field::Field, record::Record};
use std::error::Error;
use value::ValueWrapper;

mod value;

pub struct ParameterizedQuery {
    pub sql: String,
    pub params: Vec<ValueWrapper>,
}

pub fn generate_insert_statement(
    table_name: &str,
    record: &Record,
) -> Result<ParameterizedQuery, Box<dyn Error>> {
    let mut column_names = Vec::new();
    let mut param_placeholders = Vec::new();
    let mut params = Vec::new();

    let mut index = 1;
    for field in record.fields() {
        column_names.push(field.name());
        param_placeholders.push(format!("${index}").to_string());
        params.push(ValueWrapper(field.value()));
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

#[allow(dead_code)]
pub fn generate_update_statement(
    table_name: &str,
    record: &Record,
    unique_fields: &Vec<String>,
) -> Result<ParameterizedQuery, Box<dyn Error>> {
    // Separate fields into unique and non-unique
    let non_unique_fields: Vec<&Field> = record
        .fields()
        .iter()
        .filter(|field| !unique_fields.contains(&field.name().to_string()))
        .collect();

    // Generate SET clause for non-unique fields
    let set_clause = non_unique_fields
        .iter()
        .enumerate()
        .map(|(i, field)| format!("{} = ${}", field.name(), i + unique_fields.len() + 1))
        .collect::<Vec<String>>()
        .join(", ");

    // Generate WHERE clause for unique fields
    let where_clause = unique_fields
        .iter()
        .enumerate()
        .map(|(i, name)| format!("{} = ${}", name, i + 1))
        .collect::<Vec<String>>()
        .join(" AND ");

    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        table_name, set_clause, where_clause
    );

    let mut params = Vec::new();
    for field in record.fields() {
        params.push(ValueWrapper(field.value()));
    }

    Ok(ParameterizedQuery { sql, params })
}

#[cfg(test)]
mod tests;
