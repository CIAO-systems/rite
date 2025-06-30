use regex::Regex;

use super::*;

#[test]
fn test_adder_invalid_parameter() -> Result<(), Box<dyn std::error::Error>> {
    let adder = Adder::new("number:autoinc:additional_parameter");
    assert!(adder.is_err());
    let err = adder.unwrap_err();

    assert_eq!(
        "Invalid parameter format: Expected 'field:function', got 'number:autoinc:additional_parameter'",
        err.to_string()
    );
    Ok(())
}

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

fn is_valid_uuid_format(input: &str) -> bool {
    let uuid_regex =
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    uuid_regex.is_match(input)
}

#[test]
fn test_uuid() -> Result<(), Box<dyn std::error::Error>> {
    let adder = Adder::new("number:uuid")?;

    // Test UUID format
    let value1 = adder.value();
    if let Value::String(uuid) = value1.clone() {
        assert!(is_valid_uuid_format(&uuid));
    } else {
        panic!("Wrong data type");
    }

    let value2 = adder.value();
    assert_ne!(value1, value2);
    Ok(())
}

#[test]
fn test_unknown_function() -> Result<(), Box<dyn std::error::Error>> {
    let adder = Adder::new("number:unknown_function");

    assert!(adder.is_err());
    let err = adder.unwrap_err();

    assert_eq!("Unknown function: 'unknown_function'", err.to_string());
    Ok(())
}

#[test]
fn test_name() -> Result<(), Box<dyn std::error::Error>> {
    let adder = Adder::new("number:uuid")?;
    assert_eq!("number", adder.name());
    Ok(())
}

#[test]
fn test_empty() -> Result<(), Box<dyn std::error::Error>> {
    let adder = Adder::new("field:empty")?;

    let value = adder.value();
    if let Value::String(s) = value {
        assert!(s.is_empty());
    } else {
        panic!("Empty value is not a string");
    }
    Ok(())
}

#[test]
fn test_new_known_no_arg_functions() {
    let test_cases = vec![
        ("field:autoinc", "autoinc"),
        ("field:uuid", "uuid"),
        ("field:empty", "empty"),
        ("field:now", "now"), // Test 'now' without parentheses
    ];

    for (input, expected_name) in test_cases {
        let adder = Adder::new(input).unwrap();
        assert_eq!(adder.name(), "field");
        assert!(
            matches!(adder.get_type(), AdderType::Function { name, args } if name == expected_name && args.is_empty()),
            "Failed for input: {}",
            input
        );
    }
}

#[test]
fn test_new_known_param_functions() {
    let test_cases = vec![
        ("message:value('hello world')", "value", vec!["hello world"]),
    ];

    for (input, expected_name, expected_args) in test_cases {
        let adder = Adder::new(input).unwrap();
        assert_eq!(adder.name(), input.split(':').next().unwrap()); // Use input field name
        assert!(
            matches!(adder.get_type(), AdderType::Function { name, args }
                     if name == expected_name && args.iter().map(|s| s.as_str()).collect::<Vec<&str>>() == expected_args),
            "Failed for input: {}",
            input
        );
    }
}

#[test]
fn test_new_func_with_whitespace_args() {
    let adder = Adder::new("trim_test:value ( 'a' )").unwrap();
    assert_eq!(adder.name(), "trim_test");
    assert!(
        matches!(adder.get_type(), AdderType::Function { name, args }
            if name == "value" && args == vec!["a".to_string()]
        )
    );
}

#[test]
fn test_new_error_invalid_format_no_colon() {
    let result = Adder::new("just_a_field");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid parameter format: Expected 'field:function', got 'just_a_field'"
    );
}

#[test]
fn test_new_error_empty_function_string() {
    let result = Adder::new("field:");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid or empty function name in ''"
    );
}

#[test]
fn test_new_error_empty_function_name_with_parens() {
    let result = Adder::new("field:()");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid or empty function name in '()'"
    );
}

#[test]
fn test_new_error_unexpected_char_after_func_name() {
    let result = Adder::new("field:value trailing_chars");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid characters after function name 'value' in 'value trailing_chars'"
    );
}

#[test]
fn test_new_error_missing_closing_paren() {
    let result = Adder::new("field:sum(arg1, 'arg2'");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Missing closing parenthesis"
    );
}

#[test]
fn test_new_error_unexpected_closing_paren() {
    let result = Adder::new("field:func)");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unexpected closing parenthesis in function 'func)'"
    );
}

#[test]
fn test_new_error_malformed_string_literal_in_args() {
    let result = Adder::new("field:sum('a,b)"); // Missing closing single quote, using 'sum'
    println!("{:?}", result);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Missing closing parenthesis"
    );
}

// --- Tests for Adder::new() - ERROR CASES (Unknown Function) ---

#[test]
fn test_new_error_unknown_no_arg_function() {
    let result = Adder::new("field:unknown_func");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unknown function: 'unknown_func'"
    );
}

#[test]
fn test_new_error_unknown_param_function() {
    let result = Adder::new("field:calculate(1,2)");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unknown function with parameters: 'calculate'"
    );
}

// --- Tests for Adder::value() - SUCCESS CASES ---

#[test]
fn test_value_autoinc_and_state() {
    let adder = Adder::new("invoice_num:autoinc").unwrap();
    assert_eq!(adder.value(), Value::I32(1));
    assert_eq!(adder.value(), Value::I32(2));
    assert_eq!(adder.auto_inc_last_value.borrow()["invoice_num"], 2); // Directly check internal state
}

#[test]
fn test_value_uuid_uniqueness() {
    let adder = Adder::new("unique_id:uuid").unwrap();
    let val1 = adder.value();
    let val2 = adder.value();
    assert!(matches!(val1, Value::String(_)));
    assert!(matches!(val2, Value::String(_)));
    assert_ne!(val1, val2); // UUIDs should almost certainly be unique
    if let (Value::String(s1), Value::String(s2)) = (val1, val2) {
        assert!(Uuid::parse_str(&s1).is_ok());
        assert!(Uuid::parse_str(&s2).is_ok());
    }
}

#[test]
fn test_value_empty_string() {
    let adder = Adder::new("empty_field:empty").unwrap();
    assert_eq!(adder.value(), Value::String("".to_string()));
}

#[test]
fn test_value_value_with_quotes() {
    let adder = Adder::new("val:value('Hello, Rust!')").unwrap();
    assert_eq!(adder.value(), Value::String("Hello, Rust!".to_string()));
}

#[test]
fn test_value_value_unquoted_string() {
    let adder = Adder::new("val:value(unquoted_text)").unwrap();
    assert_eq!(adder.value(), Value::String("unquoted_text".to_string()));
}

#[test]
fn test_value_now_format() {
    let adder = Adder::new("current_ts:now").unwrap();
    let val = adder.value();
    assert!(matches!(val, Value::String(_)));
    if let Value::String(s) = val {
        // Very basic check that it's a non-empty string
        assert!(!s.is_empty());
    }
}

// Removed test_value_log_all_arg_types

// --- Tests for Adder::value() - ERROR CASES (Returns Value::String) ---

#[test]
fn test_value_value_error_no_arguments() {
    let adder = Adder::new("val:value()").unwrap();
    let result = adder.value();
    assert!(
        matches!(result, Value::String(s) if s.contains("Error: Value function requires at least one argument"))
    );
}

#[test]
fn test_value_empty_name_function_call() {
    // This case is now caught by `new` as an error.
    let result = Adder::new("test:()");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid or empty function name in '()'"
    );
}

#[test]
fn test_value() {
    let result = Adder::new("timestamp.timeZone:value('Europe/Berlin')");
    assert!(result.is_ok());
    let value = result.unwrap().value();
    assert_eq!(
        Value::String("Europe/Berlin".to_string()),
        value,
        "Expected 'Europe/Berlin' got '{}'",
        value
    );
}
