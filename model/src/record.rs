use crate::field::Field;

#[derive(Debug)]
pub struct Record {
    fields: Vec<Field>,
}

impl Record {
    pub fn new() -> Self {
        Self { fields: vec![] }
    }

    pub fn copy(other: &Record) -> Self {
        Self {
            fields: other.fields.clone(),
        }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn fields_as_mut(&mut self) -> &mut Vec<Field> {
        &mut self.fields
    }

    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == name)
    }

    pub fn field_by_name_mut(&mut self, name: &str) -> Option<&mut Field> {
        self.fields.iter_mut().find(|field| field.name() == name)
    }
}

#[cfg(test)]
mod tests;
