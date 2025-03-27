use super::*;

#[test]
fn test_get_config_value_success() {
    let mut config = Configuration::new();
    config.insert("port".to_string(), "8080".to_string());

    let port: Option<u16> = get_config_value(&Some(config.clone()), "port");
    assert_eq!(port, Some(8080));

    let value: Option<String> = get_config_value(&Some(config.clone()), "port");
    assert_eq!(value, Some("8080".to_string()));

    let enabled: Option<bool> = get_config_value(&Some(config), "enabled"); // Not present
    assert_eq!(enabled, None);

    let mut config = Configuration::new();
    config.insert("enabled".to_string(), "true".to_string());
    let enabled: Option<bool> = get_config_value(&Some(config), "enabled"); // Present
    assert_eq!(enabled, Some(true));
}

#[test]
fn test_get_config_value_missing_key() {
    let config = Configuration::new();
    let value: Option<String> = get_config_value(&Some(config), "missing_key");
    assert_eq!(value, None);
}

#[test]
fn test_get_config_value_empty_config() {
    let value: Option<String> = get_config_value(&None, "any_key");
    assert_eq!(value, None);
}

#[test]
fn test_get_config_value_invalid_type() {
    let mut config = Configuration::new();
    config.insert("port".to_string(), "not_a_number".to_string());

    let port: Option<u16> = get_config_value(&Some(config), "port");
    assert_eq!(port, None); // Parsing should fail

    let mut config = Configuration::new();
    config.insert("float".to_string(), "3.14".to_string());
    let float_val: Option<f32> = get_config_value(&Some(config), "float");
    assert_eq!(float_val, Some(3.14));
}

#[test]
fn test_get_config_value_with_spaces() {
    let mut config = Configuration::new();
    config.insert(
        "key_with_spaces".to_string(),
        "  value with spaces  ".to_string(),
    );

    let value: Option<String> = get_config_value(&Some(config), "key_with_spaces");
    assert_eq!(value, Some("  value with spaces  ".to_string())); // Preserves spaces

    let mut config = Configuration::new();
    config.insert("key_with_spaces_trimmed".to_string(), "  123  ".to_string());
    let value_trimmed: Option<u32> = get_config_value(&Some(config), "key_with_spaces_trimmed");
    assert_eq!(value_trimmed, None); // spaces dont parse
}

#[test]
fn test_insert_and_get_methods() {
    let mut config = Configuration::new();

    // Insert various types of string values
    config.insert_str("name", "John Doe");
    config.insert_str("age", "30");
    config.insert_str("numbers", "1,2,3,4,5");
    config.insert_str("enabled", "true");

    // Verify inserted values
    assert_eq!(config.get("name"), Some("John Doe".to_string()));
    assert_eq!(config.get("age"), Some("30".to_string()));

    // Test get_list method with inserted value
    let numbers: Option<Vec<i32>> = config.get_list("numbers");
    assert_eq!(numbers, Some(vec![1, 2, 3, 4, 5]));

    // Test get_bool method with inserted value
    assert_eq!(config.get_bool("enabled"), Some(true));
}

#[test]
fn test_overwriting_existing_values() {
    let mut config = Configuration::new();

    // Initial insertion
    config.insert_str("flag", "true");
    assert_eq!(config.get_bool("flag"), Some(true));

    // Overwrite with a different value
    config.insert_str("flag", "false");
    assert_eq!(config.get_bool("flag"), Some(false));

    // Overwrite with an invalid boolean
    config.insert_str("flag", "invalid");
    assert_eq!(config.get_bool("flag"), None);

    // Verify that only one item exists for the key
    assert_eq!(
        config
            .config
            .unwrap()
            .iter()
            .filter(|item| item.key == "flag")
            .count(),
        1
    );
}

#[test]
fn test_multiple_insertions() {
    let mut config = Configuration::new();

    // Insert multiple different keys
    config.insert_str("key1", "value1");
    config.insert_str("key2", "value2");
    config.insert_str("key3", "value3");

    // Verify all keys are present
    assert_eq!(config.get("key1"), Some("value1".to_string()));
    assert_eq!(config.get("key2"), Some("value2".to_string()));
    assert_eq!(config.get("key3"), Some("value3".to_string()));

    // Verify total number of items
    assert_eq!(config.len(), 3);
}

#[test]
fn test_complex_list_insertions() {
    let mut config = Configuration::new();

    // Insert lists with various formats
    config.insert_str("integers", "1, 2, 3, 4, 5");
    config.insert_str("floats", "1.1, 2.2, 3.3");
    config.insert_str("mixed", "1, two, 3, four, 5");

    // Test integer list with spaces
    let integers: Option<Vec<i32>> = config.get_list("integers");
    assert_eq!(integers, Some(vec![1, 2, 3, 4, 5]));

    // Test float list
    let floats: Option<Vec<f64>> = config.get_list("floats");
    assert_eq!(floats, Some(vec![1.1, 2.2, 3.3]));

    // Test mixed list (should filter out non-parseable values)
    let mixed: Option<Vec<i32>> = config.get_list("mixed");
    assert_eq!(mixed, Some(vec![1, 3, 5]));
}
