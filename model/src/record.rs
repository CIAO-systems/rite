use crate::field::Field;

pub struct Record {
    fields: Vec<Field>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            fields: vec![],
        }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

#[cfg(test)]
mod tests;
