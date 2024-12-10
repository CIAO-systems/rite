use model::record::Record;
use std::error::Error;
use value::ValueWrapper;

mod value;

pub struct ParameterizedQuery {
    pub sql: String,
    pub params: Vec<ValueWrapper>,
}

pub fn generate_insert_statement(
    record: &Record,
    table_name: &str,
) -> Result<ParameterizedQuery, Box<dyn Error>> {
    let mut column_names = Vec::new();
    let mut param_placeholders = Vec::new();
    let mut params = Vec::new();

    for field in record.fields() {
        column_names.push(field.name());
        param_placeholders.push("?".to_string());
        params.push(ValueWrapper(field.value()));
    }

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table_name,
        column_names.join(", "),
        param_placeholders.join(", ")
    );

    Ok(ParameterizedQuery { sql, params })
}

pub fn _generate_create_table_statement(record: &Record, table_name: &str) -> String {
    let column_definitions: Vec<String> = record
        .fields()
        .iter()
        .map(|field| {
            let column_type = value::_get_sql_type(field.value_as_ref());
            format!(
                "{} {} {}",
                field.name().replace(' ', "_"),
                column_type,
                "NOT NULL"
            )
        })
        .collect();

    format!(
        "CREATE TABLE {} ({});",
        table_name.replace(' ', "_"),
        column_definitions.join(",")
    )
}

#[cfg(test)]
mod tests;
