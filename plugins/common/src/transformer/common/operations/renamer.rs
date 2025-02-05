use model::field::Field;


pub struct Renamer {
    from: String,
    to: String,
}

impl Renamer {
    pub fn new(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Split the date string by <from>':'<to>
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 2 {
            Err(format!("Invalid parameter: {}", data).into())
        } else {
            Ok(Self {
                from: parts[0].to_string(),
                to: parts[1].to_string(),
            })
        }
    }

    /// Apply the renaming to the field, when it matches the from value
    pub fn apply(&self, field: &Field) -> Option<&str> {
        if self.from == field.name() {
            Some(&self.to)
        } else {
            None
        }
    }
}
