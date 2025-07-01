use chrono::Local;
use model::value::Value;
use uuid::Uuid;

impl super::Adder {
    pub fn handle_autoinc(&self) -> Value {
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

    pub fn handle_uuid(&self) -> Value {
        Value::String(Uuid::new_v4().to_string())
    }

    pub fn handle_empty(&self) -> Value {
        Value::String("".to_string())
    }

    pub fn handle_value(&self, args: &Vec<String>) -> Value {
        if let Some(val_str) = args.first() {
            Value::String(val_str.clone())
        } else {
            Value::String("Error: Value function requires at least one argument".to_string())
        }
    }

    pub fn handle_now(&self) -> Value {
        Value::String(Local::now().to_string())
    }
}
