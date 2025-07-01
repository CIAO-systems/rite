use std::{cell::RefCell, collections::HashMap};

use model::value::Value;

mod functions;
mod parser;

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

impl Adder {
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
                "autoinc" => self.handle_autoinc(),
                "uuid" => self.handle_uuid(),
                "empty" => self.handle_empty(),
                "value" => self.handle_value(args),
                "now" => self.handle_now(),
                _ => Value::String(format!("Error: Unknown function: '{}'", name)),
            },
        }
    }
}

#[cfg(test)]
mod tests;
