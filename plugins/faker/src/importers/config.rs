use rand::Rng;
use serde::Deserialize;

use super::functions::{Milliseconds, RandomFunction, Timezone, Uuid};

#[derive(Debug, Deserialize, PartialEq)]
pub struct RiteRandomImport {
    pub generator: Generator,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Generator {
    #[serde(rename = "field", default)]
    pub fields: Vec<Field>,
    pub number: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Field {
    pub name: String,
    pub function: String,
    #[serde(default)]
    pub optional: Option<bool>,
}

impl Field {
    pub fn create_generator(&self) -> Option<Box<dyn RandomFunction>> {
        match self.function.as_str() {
            "milliseconds" => Some(Box::new(Milliseconds)),
            "timezone" => Some(Box::new(Timezone)),
            "uuid" => Some(Box::new(Uuid)),
            _ => None,
        }
    }

    pub fn is_needed(&self) -> bool {
        match self.optional {
            Some(true) => rand::rng().random_bool(0.5),
            _ => true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_deserialization() {
        let data = r#"
        <rite-random-import>
            <generator number="10">
                <field name="timestamp.timeUtc" function="milliseconds" />
                <field name="timestamp.timeZone" function="timezone" />
                <field name="identity.userId" function="uuid" optional="true" />
                <field name="identity.badgeId" function="uuid" optional="true" />
                <field name="deviceId" function="uuid" />
                <field name="timeTypeId" function="uuid" />
                <field name="projectId" function="uuid" />
                <field name="projectTaskId" function="uuid" />
                <field name="costcenterId" function="uuid" />
            </generator>
        </rite-random-import>
        "#;

        let expected = RiteRandomImport {
            generator: Generator {
                number: 10,
                fields: vec![
                    Field {
                        name: "timestamp.timeUtc".to_string(),
                        function: "milliseconds".to_string(),
                        optional: None,
                    },
                    Field {
                        name: "timestamp.timeZone".to_string(),
                        function: "timezone".to_string(),
                        optional: None,
                    },
                    Field {
                        name: "identity.userId".to_string(),
                        function: "uuid".to_string(),
                        optional: Some(true),
                    },
                    Field {
                        name: "identity.badgeId".to_string(),
                        function: "uuid".to_string(),
                        optional: Some(true),
                    },
                    Field {
                        name: "deviceId".to_string(),
                        function: "uuid".to_string(),
                        optional: None,
                    },
                    Field {
                        name: "timeTypeId".to_string(),
                        function: "uuid".to_string(),
                        optional: None,
                    },
                    Field {
                        name: "projectId".to_string(),
                        function: "uuid".to_string(),
                        optional: None,
                    },
                    Field {
                        name: "projectTaskId".to_string(),
                        function: "uuid".to_string(),
                        optional: None,
                    },
                    Field {
                        name: "costcenterId".to_string(),
                        function: "uuid".to_string(),
                        optional: None,
                    },
                ],
            },
        };

        let parsed: RiteRandomImport = serde_xml_rs::from_str(data).expect("Failed to parse XML");
        assert_eq!(parsed, expected);
    }
}
