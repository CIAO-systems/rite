pub mod com {
    pub mod atoss {
        pub mod atc {
            pub mod protobuf {
                tonic::include_proto!("com.atoss.atc.protobuf");
            }
        }
    }
}

pub mod connection;
pub mod importers;

#[no_mangle]
pub fn create_importer(
    name: &str,
) -> Result<Box<dyn import::Importer>, Box<dyn std::error::Error>> {
    match name {
        "absences" => Ok(Box::new(importers::absences::Absences::new())),
        "dataset" => Ok(Box::new(importers::dataset::Dataset::new())),
        "clock_records" => Ok(Box::new(importers::clock_records::ClockRecords::new())),
        _ => Err(format!("Importer not found: {name}").into()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::com::atoss::atc::protobuf::{field::Value, AbsencesRequest, Field, Record};

    #[test]
    fn test_absences() {
        let request = AbsencesRequest {
            employee_ids: vec!["01".to_string(), "02".to_string(), "03".to_string()],
            start_date: None,
            end_date: None,
            account_ids: vec![1, 2, 3],
            plan_version: -1,
            options: None,
        };

        // Asserting specific field's value
        assert_eq!(request.employee_ids.len(), 3);
        assert!(request.start_date.is_none());
        assert_eq!(request.plan_version, -1);
    }

    #[test]
    fn test_proto_record() {
        let mut record = Record {
            field: HashMap::new(),
        };

        let name = String::from("fieldname");
        let value = Value::StringValue(String::from("test value"));
        let field = Field {
            name: name.clone(),
            value: Some(value),
        };
        record.field.insert(name.clone(), field);

        let value = record.field.get(name.as_str()).unwrap();
        match &value.value {
            Some(value) => {
                println!("{:?}", value);
                match value {
                    Value::StringValue(s) => {
                        println!("{:?}", s);
                        assert_eq!("test value", s);
                    }
                    _ => panic!("Wrong type"),
                }
            }
            None => panic!("No value in field"),
        }
    }
}
