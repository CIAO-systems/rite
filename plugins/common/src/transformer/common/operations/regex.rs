use std::collections::HashMap;

use regex::Regex;

/// Apply all the regex-replacement patterns for `input`
/// # Example regex/replacements:
/// * ("a", "b") -> replace a with b
/// * ("^(.*)", "<prefix>$1") -> Add prefix
/// * ("(.*)$", "$1<suffix>") -> Add suffix
/// * ("^.{3}", "") -> replace leading 3 characters
/// * (".{3}$", "") -> replace trailing 3 characters
/// 
pub fn apply_regex_transformations(
    input: &str,
    patterns: &[(&str, &str)],
    fields: &HashMap<&str, &str>,
) -> String {
    let mut result = input.to_string();

    for (pattern, replacement) in patterns {
        let re = Regex::new(pattern).unwrap();
        result = re
            .replace_all(&result, |caps: &regex::Captures| {
                let mut replaced = replacement.to_string();
                // Replace placeholders with values from fields
                for (key, value) in fields {
                    replaced = replaced.replace(&format!("${{{}}}", key), value);
                }
                // Replace $1, $2, etc. with captured groups
                println!("{:?}", caps);
                for i in 0..caps.len() {
                    replaced = replaced.replace(&format!("${}", i), &caps[i]);
                }
                replaced
            })
            .to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::transformer::common::operations::regex::apply_regex_transformations;


    #[test]
    fn test_remove_string() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("-", ""), // Remove dashes
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "1-234-56789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("123456789", transformed);
    }

    #[test]
    fn test_replace_string() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("-", "#"), // Replace dash with hash
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "1-234-56789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("1#234#56789", transformed);
    }

    #[test]
    fn test_replace_string_with_field() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("-", "[${company_id}]"), // Replace dash with hash
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "1-234-56789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("1[XYZ]234[XYZ]56789", transformed);
    }

    #[test]
    fn test_remove_trailing() {
        // Define regex patterns and replacements
        let patterns = vec![
            (".{3}$", ""), // Remove last three characters
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "123456789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("123456", transformed);
    }

    #[test]
    fn test_remove_leading() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("^.{3}", ""), // Remove first three characters
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "123456789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("456789", transformed);
    }

    #[test]
    fn test_add_prefix() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("^(.*)", "${company_id}$1"), // Add company_id as prefix
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ-");

        let input = "1-234-56789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("XYZ-1-234-56789", transformed);
    }

    #[test]
    fn test_add_suffix() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("(.*)$", "$1-${company_id}"), // Add company_id as prefix
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "1-234-56789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("1-234-56789-XYZ", transformed);
    }

    #[test]
    fn test_all() {
        // Define regex patterns and replacements
        let patterns = vec![
            ("-", ""),                     // Remove dashes
            ("^.{3}", ""),                 // Remove first three characters
            ("^(.*)", "${company_id}-$1"), // Add company_id as prefix
            ("(.*)$", "$1-${company_id}"), // Add company_id as suffix
        ];

        // Define fields to be used in replacements
        let mut fields = HashMap::new();
        fields.insert("company_id", "XYZ");

        let input = "1-234-56789";
        let transformed = apply_regex_transformations(input, &patterns, &fields);
        assert_eq!("XYZ-456789-XYZ", transformed);
    }
}
