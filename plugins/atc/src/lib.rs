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
        "dataset" => Ok(Box::new(importers::dataset::Dataset::new())),
        _ => Err("Not implemented".into()),
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::com::atoss::atc::protobuf::{field::Value, Field, Record};

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
