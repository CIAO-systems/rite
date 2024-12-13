use std::{cell::RefCell, collections::HashMap};

use model::value::Value;
use uuid::Uuid;

enum AdderType {
    AutoInc,
    Uuid,
}

pub struct Adder {
    name: String,
    adder_type: AdderType,
    // for the auto inc field
    auto_inc_last_value: RefCell<HashMap<String, i32>>,
}

impl Adder {
    pub fn new(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 2 {
            Err(format!("Invalid parameter: {}", data).into())
        } else {
            Ok(Self {
                name: parts[0].to_string(),
                adder_type: match parts[1] {
                    "autoinc" => AdderType::AutoInc,
                    "uuid" => AdderType::Uuid,
                    _ => return Err(format!("Unknown type: {}", parts[1]).into()),
                },
                auto_inc_last_value: RefCell::new(HashMap::new()),
            })
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn value(&self) -> model::value::Value {
        match self.adder_type {
            AdderType::AutoInc => {
                let value = *self
                    .auto_inc_last_value
                    .borrow()
                    .get(&self.name)
                    .unwrap_or(&0) + 1;

                let mut map = self.auto_inc_last_value.borrow_mut();
                map.insert(self.name.clone(), value);
                Value::I32(value)
            }
            AdderType::Uuid => Value::String(Uuid::new_v4().to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() -> Result<(), Box<dyn std::error::Error>> {
        let adder = Adder::new("number:autoinc")?;

        // Test initial value
        assert_eq!(adder.value(), Value::I32(1));

        // Test incrementation
        assert_eq!(adder.value(), Value::I32(2));
        assert_eq!(adder.value(), Value::I32(3));

        // Test with a different name
        let adder2 = Adder::new("test2:autoinc")?;

        assert_eq!(adder2.value(), Value::I32(1));
        assert_eq!(adder2.value(), Value::I32(2));

        // Ensure original adder still increments correctly
        assert_eq!(adder.value(), Value::I32(4));

        Ok(())
    }
}
