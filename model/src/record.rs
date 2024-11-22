use crate::field::Field;

#[derive(Debug)]
pub struct Record {
    fields: Vec<Field>,
}

impl Record {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    pub fn fields(&mut self) -> &mut Vec<Field> {
        &mut self.fields
    }

    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == name)
    }
}

#[cfg(test)]
mod tests;
