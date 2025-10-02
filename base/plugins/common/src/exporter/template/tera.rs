use model::record::Record;

/// Converts a model Record in a Tera object
pub fn record_to_tera_object(record: &Record) -> tera::Map<String, tera::Value> {
    let mut result = tera::Map::new();
    for field in record.fields() {
        let name = field.name().to_string();
        let value = model_to_tera(field.value());
        result.insert(name, value);
    }
    result
}

/// Converts a model Value to a Tera Value
fn model_to_tera(value: model::value::Value) -> tera::Value {
    match value {
        model::value::Value::Record(r) => tera::Value::Object(record_to_tera_object(&r)),
        model::value::Value::Collection(v) => {
            let array: Vec<tera::Value> = v.into_iter().map(|mv| model_to_tera(mv)).collect();
            tera::Value::Array(array)
        }

        model::value::Value::I8(i) => tera::Value::Number(i.into()),
        model::value::Value::I16(i) => tera::Value::Number(i.into()),
        model::value::Value::I32(i) => tera::Value::Number(i.into()),
        model::value::Value::I64(i) => tera::Value::Number(i.into()),
        model::value::Value::U8(i) => tera::Value::Number(i.into()),
        model::value::Value::U16(i) => tera::Value::Number(i.into()),
        model::value::Value::U32(i) => tera::Value::Number(i.into()),
        model::value::Value::U64(i) => tera::Value::Number(i.into()),
        model::value::Value::F32(f) => tera::Value::Number(
            tera::Number::from_f64(f as f64)
                .unwrap_or_else(|| tera::Number::from_f64(f64::MAX).unwrap()),
        ),
        model::value::Value::F64(f) => tera::Value::Number(
            tera::Number::from_f64(f as f64)
                .unwrap_or_else(|| tera::Number::from_f64(f64::MAX).unwrap()),
        ),
        _ => tera::Value::String(value.to_string()),
    }
}

#[cfg(test)]
mod tests;
