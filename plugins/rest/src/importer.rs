use import::Importer;
use model::{field::Field, record::Record, Initializable};

pub static CONFIG_URL: &str = "url";
pub static CONFIG_RECORDS_FIELD: &str = "records_field";

pub struct RESTImporter {
    url: Option<String>,
    records_field: Option<String>,
}

impl RESTImporter {
    pub fn new() -> Self {
        RESTImporter {
            url: None,
            records_field: None,
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
        }

        Ok(())
    }
}

impl Importer for RESTImporter {
    fn read(&mut self, callback: import::RecordCallback) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref url) = self.url {
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()?.text()?;
            let json = json::parse(&response)?;

            let records_array = match self.records_field {
                Some(ref records_field) => &json[records_field],
                None => &json,
            };

            for json_record in records_array.members() {
                let record = record_from_json(json_record);
                callback(&record);
            }
        }

        Ok(())
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

fn record_from_json(json: &json::JsonValue) -> Record {
    let mut record = Record::new();
    match json {
        json::JsonValue::Object(object) => {
            for (key, value) in object.iter() {
                match value {
                    json::JsonValue::Short(short) => {
                        let field = Field::new_string(key.to_string(), short.to_string());
                        record.fields_as_mut().push(field);
                    }
                    json::JsonValue::String(s) => {
                        let field = Field::new_string(key.to_string(), s.to_string());
                        record.fields_as_mut().push(field);
                    }
                    json::JsonValue::Number(number) => {
                        let field = Field::new_string(key.to_string(), number.to_string());
                        record.fields_as_mut().push(field);
                    }
                    json::JsonValue::Boolean(b) => {
                        let field = Field::new_bool(key.to_string(), b.to_owned());
                        record.fields_as_mut().push(field);
                    }
                    _ => {}
                }
            }
        }
        _ => {
            let field = Field::new_string("json".to_string(), json.to_string());
            record.fields_as_mut().push(field);
        }
    }

    record
}

#[cfg(test)]
mod tests {
    use std::fs;

    use import::Importer;
    use model::{
        field::Field, record::Record, value::Value, xml::config::Configuration, Initializable,
    };

    use super::{record_from_json, RESTImporter, CONFIG_RECORDS_FIELD, CONFIG_URL};

    static EXPECTED: [(&'static str, &'static str); 6] = [
        ("4", "A New Hope"),
        ("5", "The Empire Strikes Back"),
        ("6", "Return of the Jedi"),
        ("1", "The Phantom Menace"),
        ("2", "Attack of the Clones"),
        ("3", "Revenge of the Sith"),
    ];

    #[test]
    fn test_record_from_json() {
        // Create a sample JSON object to test
        let sample_json = json::parse(
            r#"
        {
            "name": "Tatooine",
            "climate": "arid",
            "population": "200000",
            "features": ["desert", "canyons"],
            "details": {
                "rotation_period": 23,
                "orbital_period": 304
            }
        }
        "#,
        )
        .unwrap();

        let record = record_from_json(&sample_json);
        println!("{:?}", record);

        let field_option = record.field_by_name("name");
        assert!(field_option.is_some());
        if let Some(field) = field_option {
            assert_eq!("name", field.name());
            if let Value::String(value) = field.value() {
                assert_eq!("Tatooine", value);
            }
        }

        let field_option = record.field_by_name("climate");
        assert!(field_option.is_some());
        if let Some(field) = field_option {
            assert_eq!("climate", field.name());
            if let Value::String(value) = field.value() {
                assert_eq!("arid", value);
            }
        }

        let field_option = record.field_by_name("population");
        assert!(field_option.is_some());
        if let Some(field) = field_option {
            assert_eq!("population", field.name());
            if let Value::String(value) = field.value() {
                assert_eq!("200000", value);
            }
        }
    }

    #[test]
    fn test_records_in_results() -> Result<(), Box<dyn std::error::Error>> {
        let json = json::parse(&fs::read_to_string(
            "../../data/test/records-in-results.json",
        )?)?;
        let results = &json["results"];

        assert_results(EXPECTED, results);

        Ok(())
    }

    #[test]
    fn test_records_in_root() -> Result<(), Box<dyn std::error::Error>> {
        let json = json::parse(&fs::read_to_string("../../data/test/records-in-root.json")?)?;

        assert_results(EXPECTED, &json);

        Ok(())
    }

    fn create_importer() -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
        let mut importer = RESTImporter::new();
        let mut config = Configuration::new();
        config.insert(
            CONFIG_URL.to_string(),
            "https://swapi.dev/api/films".to_string(),
        );
        config.insert(CONFIG_RECORDS_FIELD.to_string(), "results".to_string());

        importer.init(Some(config))?;

        Ok(Box::new(importer))
    }

    #[test]
    fn test_importer_in_results() -> Result<(), Box<dyn std::error::Error>> {
        let mut records = Vec::new();
        let mut importer = create_importer()?;
        importer.read(&mut |record| {
            records.push(Record::copy(&record));
        })?;

        assert_result_records(EXPECTED, &records);

        Ok(())
    }

    fn assert_result_records(expected: [(&str, &str); 6], results: &Vec<Record>) {
        let mut index = 0;
        for record in results {
            let episode = record
                .field_by_name("episode_id")
                .unwrap_or(&Field::default())
                .value();
            let title = record
                .field_by_name("title")
                .unwrap_or(&Field::default())
                .value();
            let values = expected[index];
            assert_eq!(values.0, episode.to_string());
            assert_eq!(values.1, title.to_string());

            println!("Episode {} = {}", episode, title,);
            index += 1;
        }
    }

    fn assert_results(expected: [(&str, &str); 6], results: &json::JsonValue) {
        assert_eq!(6, results.len());
        if let json::JsonValue::Array(array) = results {
            let records: Vec<Record> = array
                .iter()
                .map(|json_record| record_from_json(json_record))
                .collect();

            assert_result_records(expected, &records);
        }
    }
}
