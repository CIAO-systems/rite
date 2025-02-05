pub struct Ignorer {
    fields: Vec<String>,
}

impl Ignorer {
    pub fn new(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            fields: data.split(',').map(String::from).collect(),
        })
    }

    pub fn apply(&self, name: &str) -> bool {
        self.fields.contains(&name.to_string())
    }
}

pub struct Includer {
    fields: Vec<String>,
}

impl Includer {
    pub fn new(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            fields: data.split(',').map(String::from).collect(),
        })
    }

    pub fn apply(&self, name: &str) -> bool {
        self.fields.contains(&name.to_string())
    }
}
