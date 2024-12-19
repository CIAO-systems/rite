use import::Importer;
use json_dotpath::DotPaths;
use model::{field::Field, record::Record, Initializable};

pub static CONFIG_URL: &str = "url";
pub static CONFIG_RECORDS_FIELD: &str = "records_field";
pub static CONFIG_FIELDS_PATH: &str = "fields_path";

pub struct RESTImporter {
    url: Option<String>,
    records_field: Option<String>,
    fields_path: Option<String>,
}

impl RESTImporter {
    pub fn new() -> Self {
        RESTImporter {
            url: None,
            records_field: None,
            fields_path: None,
        }
    }
}

impl Initializable for RESTImporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            self.url = config.get(CONFIG_URL);
            self.records_field = config.get(CONFIG_RECORDS_FIELD);
            self.fields_path = config.get(CONFIG_FIELDS_PATH);
        }

        Ok(())
    }
}

impl Importer for RESTImporter {
    fn read(&mut self, callback: import::RecordCallback) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref url) = self.url {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()?;
            let status = response.status();
            if status.is_success() {
                let text = response.text()?;
                let json: serde_json::Value = serde_json::from_str(&text)?;

                let records_array = match self.records_field {
                    Some(ref records_field) => json
                        .get(records_field)
                        .ok_or_else(|| format!("Field '{}' not found in JSON", records_field))?,
                    None => &json,
                };

                if let serde_json::Value::Array(array) = records_array {
                    for json_record in array {
                        let record = record_from_json(json_record, &self.fields_path);
                        callback(&record);
                    }
                }
            } else {
                return Err(format!(
                    "GET {} responded with HTTP status code {}",
                    url,
                    status.as_str()
                )
                .into());
            }
        }

        Ok(())
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

fn record_from_json(raw_json: &serde_json::Value, fields_path: &Option<String>) -> Record {
    let json_record = match fields_path {
        Some(fields_path) => raw_json
            .dot_get::<serde_json::Value>(fields_path)
            .ok()
            .flatten()
            .unwrap_or_else(|| {
                println!("Path '{}' not found, using raw JSON.", fields_path);
                raw_json.clone()
            }),
        None => raw_json.clone(),
    };

    let mut record = Record::new();
    match json_record {
        serde_json::Value::Object(object) => {
            for (key, value) in object.iter() {
                match value {
                    serde_json::Value::String(s) => {
                        let field = Field::new_string(key.to_string(), s.to_string());
                        record.fields_as_mut().push(field);
                    }
                    serde_json::Value::Number(number) => {
                        let field = Field::new_string(key.to_string(), number.to_string());
                        record.fields_as_mut().push(field);
                    }
                    serde_json::Value::Bool(b) => {
                        let field = Field::new_bool(key.to_string(), *b);
                        record.fields_as_mut().push(field);
                    }
                    _ => {}
                }
            }
        }
        _ => {
            let field = Field::new_string("json".to_string(), json_record.to_string());
            record.fields_as_mut().push(field);
        }
    }

    record
}

#[cfg(test)]
mod tests;
