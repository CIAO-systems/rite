use model::record::Record;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Patterns {
    pub pattern: Vec<Pattern>,
}

impl Patterns {
    pub(crate) fn apply(&self, input: &str, record: Option<&Record>) -> String {
        let mut result = input.to_string();

        for pattern in &self.pattern {
            let re = Regex::new(&pattern.matcher).unwrap();
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    let mut replaced = pattern.replacement.to_string();

                    if let Some(record) = record {
                        // Replace placeholders with values from fields
                        for field in record.fields() {
                            replaced = replaced.replace(
                                &format!("${{{}}}", field.name()),
                                &field.value().to_string(),
                            );
                        }
                    }

                    // Replace $1, $2, etc. with captured groups
                    for i in 0..caps.len() {
                        replaced = replaced.replace(&format!("${}", i), &caps[i]);
                    }
                    replaced
                })
                .to_string();
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pattern {
    pub matcher: String,
    pub replacement: String,
}

#[cfg(test)]
mod tests {
    use model::{field::add_field, record::Record, value::Value};

    use super::{Pattern, Patterns};

    #[test]
    fn test_remove_string() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "-".to_string(),
            replacement: "".to_string(),
        });

        assert_eq!("123456", patterns.apply("1-23-456", None));
    }

    #[test]
    fn test_replace_string_with_field() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "PLACEHOLDER".to_string(),
            replacement: "${company_id}".to_string(),
        });

        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "company_id",
            Value::String("ACME Inc.".to_string()),
        );
        assert_eq!(
            "ACME Inc.: 1-23-456",
            patterns.apply("PLACEHOLDER: 1-23-456", Some(&record))
        );
    }

    #[test]
    fn test_add_prefix() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "^(.*)".to_string(),
            replacement: "${prefix}: $1".to_string(),
        });

        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "prefix",
            Value::String("PRE".to_string()),
        );
        assert_eq!("PRE: 1-23-456", patterns.apply("1-23-456", Some(&record)));
    }

    #[test]
    fn test_add_suffix() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "(.*)$".to_string(),
            replacement: "$1-${suffix}".to_string(),
        });

        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "suffix",
            Value::String("SUF".to_string()),
        );
        assert_eq!("1-23-456-SUF", patterns.apply("1-23-456", Some(&record)));
    }

    #[test]
    fn test_multiple() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "-".to_string(),
            replacement: "".to_string(),
        });
        patterns.pattern.push(Pattern {
            matcher: "^(.*)".to_string(),
            replacement: "${prefix}: $1".to_string(),
        });
        patterns.pattern.push(Pattern {
            matcher: "(.*)$".to_string(),
            replacement: "$1-${suffix}".to_string(),
        });
        patterns.pattern.push(Pattern {
            matcher: "PLACEHOLDER".to_string(),
            replacement: "${company_id}".to_string(),
        });

        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "suffix",
            Value::String("SUF".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "prefix",
            Value::String("PRE".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "company_id",
            Value::String("ACME Inc.".to_string()),
        );
        assert_eq!(
            "PRE: 123(ACME Inc.)456-SUF",
            patterns.apply("1-23(PLACEHOLDER)-456", Some(&record))
        );
    }

    #[test]
    fn test_remove_leading() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "^.{3}".to_string(),
            replacement: "".to_string(),
        });

        assert_eq!("-456", patterns.apply("123-456", None));
    }

    #[test]
    fn test_remove_trailing() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: ".{3}$".to_string(),
            replacement: "".to_string(),
        });

        assert_eq!("123-", patterns.apply("123-456", None));
    }

    #[test]
    fn test_remove_prefix() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "^ABC".to_string(),
            replacement: "".to_string(),
        });

        assert_eq!("-456", patterns.apply("ABC-456", None));
    }

    #[test]
    fn test_remove_suffix() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "ABC$".to_string(),
            replacement: "".to_string(),
        });

        assert_eq!("123-", patterns.apply("123-ABC", None));
    }

    #[test]
    fn test_add_and_remove_suffix() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "(.*)$".to_string(),
            replacement: "$1-ABC".to_string(),
        });
        patterns.pattern.push(Pattern {
            matcher: "-ABC$".to_string(),
            replacement: "".to_string(),
        });

        assert_eq!("123", patterns.apply("123", None));
    }

    #[test]
    fn test_find_and_double() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "ABC".to_string(),
            replacement: "$0-$0".to_string(),
        });

        assert_eq!("ABC-ABC-123-ABC-ABC-456", patterns.apply("ABC-123-ABC-456", None));
    }

    #[test]
    fn test_words() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "([A-Z])\\w+".to_string(),
            replacement: "\"$0\"".to_string(),
        });

        assert_eq!("\"ABC\"-123-\"ABC\"-456", patterns.apply("ABC-123-ABC-456", None));
        assert_eq!("abc-123-\"Xyz\"-456", patterns.apply("abc-123-Xyz-456", None));
    }

    #[test]
    fn test_combine() {
        let mut patterns = Patterns {
            pattern: Vec::new(),
        };
        patterns.pattern.push(Pattern {
            matcher: "".to_string(),
            replacement: "${firstname} ${lastname}".to_string(),
        });

        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "firstname",
            Value::String("Han".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "lastname",
            Value::String("Solo".to_string()),
        );
        assert_eq!(
            "Han Solo",
            patterns.apply("", Some(&record))
        );
    }

}
