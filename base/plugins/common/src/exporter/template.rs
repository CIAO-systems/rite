use ::tera::{Context, Tera, Value};
use model::export::Exporter;
use model::Initializable;
use std::error::Error;

const CFG_TEMPLATE_FILE: &str = "templateFile";
const CFG_OUTPUT_FILE: &str = "outputFile";
const TEMPLATE_NAME: &str = "template";

mod tera;

pub struct TemplateExporter {
    template_file: Option<String>,
    output_file: Option<String>,
    records: Option<Vec<::tera::Map<String, Value>>>,
}

impl TemplateExporter {
    pub(crate) fn new() -> Self {
        Self {
            template_file: None,
            output_file: None,
            records: None,
        }
    }

    /// Write the file to `output_file` after rendering it with `template_file`
    fn write_file(&self) -> Result<(), model::BoxedError> {
        if let Some(ref template_file) = self.template_file {
            let mut tera = Tera::default();
            // Load a single template file
            tera.add_template_file(template_file, Some(TEMPLATE_NAME))?;

            let mut context = Context::new();
            context.insert("records", &self.records);

            match tera.render(TEMPLATE_NAME, &context) {
                Ok(rendered) => {
                    // write rendered string in output file
                    if let Some(ref output_file) = self.output_file {
                        std::fs::write(output_file, rendered)?;
                    }
                }
                Err(e) => {
                    let error = if let Some(source) = e.source() {
                        source
                    } else {
                        &e
                    };

                    let msg = format!("Error rendering {template_file}: {error}");
                    log::error!("{msg}");
                    return Err(msg.into());
                }
            }
        }
        Ok(())
    }
}

impl Initializable for TemplateExporter {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), model::BoxedError> {
        if let Some(config) = config {
            self.template_file = config.get(CFG_TEMPLATE_FILE);
            self.output_file = config.get(CFG_OUTPUT_FILE);
        }
        Ok(())
    }
}

impl Exporter for TemplateExporter {
    fn write(&mut self, record: &model::record::Record) -> Result<(), model::BoxedError> {
        if let Some(ref mut records) = self.records {
            records.push(tera::record_to_tera_object(record));
        }
        Ok(())
    }

    fn event(&mut self, signal: model::export::Signal) -> Result<(), model::BoxedError> {
        match signal {
            model::export::Signal::Start => {
                self.records = Some(Vec::new());
                Ok(())
            }
            model::export::Signal::End => self.write_file(),
        }
    }
}

#[cfg(test)]
mod tests;
