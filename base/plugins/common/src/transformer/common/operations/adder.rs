use std::{cell::RefCell, collections::HashMap};

use chrono::Local;
use model::{BoxedError, value::Value};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AdderType {
    Function { name: String, args: Vec<String> },
}

#[derive(Debug)]
pub struct Adder {
    name: String,
    adder_type: AdderType,
    // for the auto inc field
    auto_inc_last_value: RefCell<HashMap<String, i32>>,
}

// --- Constants for Allowed Functions ---
const ALLOWED_NO_ARG_FUNCTIONS: &[&str] = &["autoinc", "uuid", "empty", "now"];
const ALLOWED_PARAM_FUNCTIONS: &[&str] = &["value"];

impl Adder {
    pub fn new(data: &str) -> Result<Self, BoxedError> {
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid parameter format: Expected 'field:function', got '{}'",
                data
            )
            .into());
        }

        let field_name = parts[0].to_string();
        let function_string = parts[1];

        let mut chars = function_string.chars().peekable();
        let mut func_name_chars = String::new();
        let mut args: Vec<String> = Vec::new();

        // 1. Parse the function name
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

        // Consume any whitespace after the function name
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next(); // Consume whitespace
            } else {
                break;
            }
        }

        // 2. Check for opening parenthesis for arguments
        let adder_type = if let Some(&c) = chars.peek() {
            if c == '(' {
                chars.next(); // Consume '('
                let mut current_arg = String::new();
                let mut in_quotes = false;
                let mut found_closing_paren = false;

                // 3. Parse arguments
                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        in_quotes = !in_quotes;
                        chars.next(); // Consume the quote
                        if !in_quotes && !current_arg.is_empty() {
                            let trimmed_arg = current_arg.trim().to_string();
                            if !trimmed_arg.is_empty() {
                                args.push(trimmed_arg);
                            }
                            current_arg.clear();
                        }
                    } else if c == ',' && !in_quotes {
                        // End of an unquoted argument
                        chars.next(); // Consume ','
                        let trimmed_arg = current_arg.trim().to_string();
                        if !trimmed_arg.is_empty() {
                            args.push(trimmed_arg);
                        }
                        current_arg.clear();
                    } else if c == ')' && !in_quotes {
                        // End of arguments list
                        chars.next(); // Consume ')'
                        let trimmed_arg = current_arg.trim().to_string();
                        if !trimmed_arg.is_empty() {
                            args.push(trimmed_arg);
                        }
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

    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> AdderType {
        self.adder_type.clone()
    }

    pub fn value(&self) -> Value {
        match &self.adder_type {
            AdderType::Function { name, args } => match name.as_str() {
                "autoinc" => {
                    let value = *self
                        .auto_inc_last_value
                        .borrow()
                        .get(&self.name)
                        .unwrap_or(&0)
                        + 1;

                    let mut map = self.auto_inc_last_value.borrow_mut();
                    map.insert(self.name.clone(), value);
                    Value::I32(value)
                }
                "uuid" => Value::String(Uuid::new_v4().to_string()),
                "empty" => Value::String("".to_string()),
                "value" => {
                    if let Some(val_str) = args.first() {
                        Value::String(val_str.clone())
                    } else {
                        Value::String(
                            "Error: Value function requires at least one argument".to_string(),
                        )
                    }
                }
                "now" => Value::String(Local::now().to_string()),
                _ => Value::String(format!("Error: Unknown function: '{}'", name)),
            },
        }
    }
}

#[cfg(test)]
mod tests;
