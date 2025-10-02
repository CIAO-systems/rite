use model::{field::Field, record::Record, xml::config::Configuration, Initializable};
use operations::{
    adder::Adder,
    filter::{Ignorer, Includer},
    formatter::Formatter,
    renamer::Renamer,
};
use model::transform::Transformer;

mod operations;

pub struct CommonTransformer {
    config: Option<Configuration>,
    adders: Vec<Adder>,
    renamers: Vec<Renamer>,
    ignorers: Vec<Ignorer>,
    includers: Vec<Includer>,
    formatters: Vec<Formatter>,
}

impl CommonTransformer {
    pub fn new() -> Self {
        Self {
            config: None,
            adders: Vec::new(),
            renamers: Vec::new(),
            ignorers: Vec::new(),
            includers: Vec::new(),
            formatters: Vec::new(),
        }
    }
}

impl Initializable for CommonTransformer {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config.clone();
        if let Some(ref config) = self.config {
            if let Some(operations) = config.as_vec_ref() {
                for item in operations {
                    match item.key.as_str() {
                        "add_field" => {
                            self.adders.push(Adder::new(&item.value)?);
                        }
                        "rename_field" => {
                            self.renamers.push(Renamer::new(&item.value)?);
                        }
                        "ignore_field" => {
                            self.ignorers.push(Ignorer::new(&item.value)?);
                        }
                        "include_field" => {
                            self.includers.push(Includer::new(&item.value)?);
                        }
                        "format_field" => self.formatters.push(Formatter::new(&item.value)?),
                        _ => continue,
                    }
                }
            }
        }

        Ok(())
    }
}

impl Transformer for CommonTransformer {
    fn process(&self, record: &Record) -> Result<Record, Box<dyn std::error::Error>> {
        let mut transformed = Record::new();

        for field in record.fields() {
            let mut name: &str = field.name();
            let mut value = field.value();

            if self.includers.iter().any(|includer| !includer.apply(name)) {
                // field should not be included
                continue;
            }

            if self.ignorers.iter().any(|ignorer| ignorer.apply(name)) {
                // field should be ignored
                continue;
            }

            // apply renamer
            for renamer in &self.renamers {
                if let Some(new_field_name) = renamer.apply(&field) {
                    name = new_field_name;
                    break;
                }
            }

            // apply formatter
            for formatter in &self.formatters {
                if let Some(new_value) = formatter.apply(&field) {
                    // Take the formatted value instead of the original
                    value = new_value;
                    break;
                }
            }

            transformed
                .fields_as_mut()
                .push(Field::new_value(name, value));
        }

        // apply adder
        for adder in &self.adders {
            transformed
                .fields_as_mut()
                .push(Field::new_value(&adder.name(), adder.value()));
        }

        return Ok(transformed);
    }
}

#[cfg(test)]
mod tests;
