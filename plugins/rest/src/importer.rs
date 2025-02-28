use import::{Importer, RecordHandler};
use json_dotpath::DotPaths;
use model::{field::Field, record::Record, value::Value, Initializable};
use reqwest::header::HeaderValue;

pub static CONFIG_URL: &str = "url";
pub static CONFIG_RECORDS_FIELD: &str = "records_field";
pub static CONFIG_FIELDS_PATH: &str = "fields_path";

pub static CONFIG_AUTH_BASIC: &str = "auth.basic";
pub static CONFIG_AUTH_BEARER: &str = "auth.bearer";
pub static CONFIG_AUTH_APIKEY: &str = "auth.api-key";

pub struct RESTImporter {
    url: Option<String>,
    records_field: Option<String>,
    fields_path: Option<String>,
    auth_basic: Option<String>,
    auth_bearer: Option<String>,
    auth_apikey: Option<String>,
}

impl RESTImporter {
    pub fn new() -> Self {
        RESTImporter {
            url: None,
            records_field: None,
            fields_path: None,
            auth_basic: None,
            auth_bearer: None,
            auth_apikey: None,
        }
    }

    /// Add the configured authentication headers
    fn setup_authentication(
        &self,
        mut request_builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        if let Some(ref user_password) = self.auth_basic {
            let (user, password) = split(user_password);
            if let Some(user) = user {
                request_builder = request_builder.basic_auth(user, password);
            }
        };

        if let Some(ref token) = self.auth_bearer {
            request_builder = request_builder.bearer_auth(token);
        };

        if let Some(ref apikey) = self.auth_apikey {
            let (key, value) = split(apikey);
            if let Some(key) = key {
                if let Some(value) = value {
                    if let Ok(value) = HeaderValue::from_str(&value) {
                        request_builder = request_builder.header(key, value);
                    }
                }
            }
        }
        request_builder
    }
}

fn split(input: &str) -> (Option<String>, Option<String>) {
    let parts: Vec<&str> = input.split(':').collect();
    let s1 = parts.get(0).map(|&s| s.to_string());
    let s2 = parts.get(1).map(|&s| s.to_string());
    (s1, s2)
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
            self.auth_apikey = config.get(CONFIG_AUTH_APIKEY);
            self.auth_basic = config.get(CONFIG_AUTH_BASIC);
            self.auth_bearer = config.get(CONFIG_AUTH_BEARER);
        }

        Ok(())
    }
}

impl Importer for RESTImporter {
    fn read(&mut self, handler: &mut dyn RecordHandler) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref url) = self.url {
            let client = reqwest::blocking::Client::new();
            let request_builder = self.setup_authentication(client.get(url));

            match request_builder.send() {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        let text = response.text()?;
                        let json: serde_json::Value = serde_json::from_str(&text)?;

                        let records_array = match self.records_field {
                            Some(ref records_field) => {
                                json.get(records_field).ok_or_else(|| {
                                    format!("Field '{}' not found in JSON", records_field)
                                })?
                            }
                            None => &json,
                        };

                        if let serde_json::Value::Array(array) = records_array {
                            for json_record in array {
                                let mut record = record_from_json(json_record, &self.fields_path);
                                handler.handle_record(&mut record)?;
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
                Err(e) => {
                    return Err(format!("Send request to URL {} ended with error: {e}", url).into());
                }
            }
        }

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
                        let field = Field::new_value(key, Value::String(s.to_string()));
                        record.fields_as_mut().push(field);
                    }
                    serde_json::Value::Number(number) => {
                        let field = Field::new_value(key, Value::String(number.to_string()));
                        record.fields_as_mut().push(field);
                    }
                    serde_json::Value::Bool(b) => {
                        let field = Field::new_value(key, Value::Bool(*b));
                        record.fields_as_mut().push(field);
                    }
                    _ => {}
                }
            }
        }
        _ => {
            let field = Field::new_value("json", Value::String(json_record.to_string()));
            record.fields_as_mut().push(field);
        }
    }

    record
}

#[cfg(test)]
mod tests;
