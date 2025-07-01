use std::{cell::RefCell, collections::HashMap};

use model::BoxedError;

use crate::transformer::common::operations::adder::AdderType;

// --- Constants for Allowed Functions ---
const ALLOWED_NO_ARG_FUNCTIONS: &[&str] = &["autoinc", "uuid", "empty", "now"];
const ALLOWED_PARAM_FUNCTIONS: &[&str] = &["value"];

impl super::Adder {
    pub fn new(data: &str) -> Result<Self, BoxedError> {
        let (field_name, function_string) = parse_parts(data)?;

        let mut chars = function_string.chars().peekable();
        let mut args: Vec<String> = Vec::new();

        // Parse the function name
        let func_name = parse_func_name(function_string, &mut chars)?;

        // Consume any whitespace after the function name
        skip_ws(&mut chars);

        // Check for opening parenthesis for arguments
        let adder_type = if let Some(&c) = chars.peek() {
            if c == '(' {
                chars.next(); // Consume '('
                let mut current_arg = String::new();
                let mut in_quotes = false;
                let mut found_closing_paren = false;

                // Parse arguments
                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        in_quotes = !in_quotes;
                        chars.next(); // Consume the quote
                        if !in_quotes && !current_arg.is_empty() {
                            push_arg(&mut args, &mut current_arg);
                        }
                    } else if c == ',' && !in_quotes {
                        // End of an unquoted argument
                        chars.next(); // Consume ','
                        push_arg(&mut args, &mut current_arg);
                    } else if c == ')' && !in_quotes {
                        // End of arguments list
                        chars.next(); // Consume ')'
                        push_arg(&mut args, &mut current_arg);
                        found_closing_paren = true;
                        break; // Exit argument parsing loop
                    } else {
                        current_arg.push(chars.next().unwrap());
                    }
                }

                if !found_closing_paren {
                    return Err("Missing closing parenthesis".into());
                }

                // Check if we correctly closed the parenthesis
                if chars.next().is_some() {
                    return Err(format!(
                        "Unexpected characters after closing parenthesis in function '{}'",
                        function_string
                    )
                    .into());
                }

                // Check if this parameterized function is known
                if !ALLOWED_PARAM_FUNCTIONS.contains(&func_name.as_str()) {
                    return Err(format!("Unknown function with parameters: '{}'", func_name).into());
                }

                AdderType::Function {
                    name: func_name,
                    args,
                }
            } else if c == ')' {
                return Err(format!(
                    "Unexpected closing parenthesis in function '{}'",
                    function_string
                )
                .into());
            } else {
                // If there's content after the function name but no '(', it's an error
                if chars.next().is_some() {
                    return Err(format!(
                        "Invalid characters after function name '{}' in '{}'",
                        func_name, function_string
                    )
                    .into());
                }
                // If no more chars, it's a function with no explicit parentheses
                // Check if this no-arg function is known
                if !ALLOWED_NO_ARG_FUNCTIONS.contains(&func_name.as_str()) {
                    return Err(format!("Unknown function: '{}'", func_name).into());
                }
                AdderType::Function {
                    name: func_name,
                    args: Vec::new(),
                }
            }
        } else {
            // No more characters after function name, so it's a no-arg function
            if !ALLOWED_NO_ARG_FUNCTIONS.contains(&func_name.as_str()) {
                return Err(format!("Unknown function: '{}'", func_name).into());
            }
            AdderType::Function {
                name: func_name,
                args: Vec::new(),
            }
        };

        Ok(Self {
            name: field_name,
            adder_type,
            auto_inc_last_value: RefCell::new(HashMap::new()),
        })
    }
}

fn parse_parts(data: &str) -> Result<(String, &str), BoxedError> {
    let parts: Vec<&str> = data.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid parameter format: Expected 'field:function', got '{}'",
            data
        )
        .into());
    }

    Ok((parts[0].to_string(), parts[1]))
}

pub fn parse_func_name(
    function_string: &str,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<String, BoxedError> {
    let mut func_name_chars = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            func_name_chars.push(chars.next().unwrap());
        } else {
            break; // End of function name
        }
    }
    let func_name = func_name_chars.to_string();
    if func_name.is_empty() {
        return Err(format!("Invalid or empty function name in '{}'", function_string).into());
    }
    Ok(func_name)
}

pub fn skip_ws(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next(); // Consume whitespace
        } else {
            break;
        }
    }
}

pub fn push_arg(args: &mut Vec<String>, current_arg: &mut String) {
    let trimmed_arg = current_arg.trim().to_string();
    if !trimmed_arg.is_empty() {
        args.push(trimmed_arg);
    }
    current_arg.clear();
}
