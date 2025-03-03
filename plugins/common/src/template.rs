use ::tera::{Context, Tera, Value};
use export::Exporter;
use model::Initializable;

const CFG_TEMPLATE_FILE: &str = "templateFile";
const CFG_OUTPUT_FILE: &str = "outputFile";

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

    fn write_file(&self) -> Result<(), model::BoxedError> {
        if let Some(ref template_file) = self.template_file {
            let mut tera = Tera::default();
            // Load a single template file
            tera.add_template_file(template_file, Some("exporter"))?;

            let mut context = Context::new();
            context.insert("records", &self.records);

            let rendered = tera.render("exporter", &context)?;

            // write rendered string in output file
            if let Some(ref output_file) = self.output_file {
                std::fs::write(output_file, rendered)?;
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

    fn event(&mut self, signal: export::Signal) -> Result<(), model::BoxedError> {
        match signal {
            export::Signal::Start => {
                self.records = Some(Vec::new());
                Ok(())
            }
            export::Signal::End => self.write_file(),
        }
    }
}

#[cfg(test)]
mod tests {
    use model::{field::add_field, record::Record};
    use tera::{Context, Tera};

    #[test]
    fn test_tera() -> Result<(), model::BoxedError> {
        let mut tera = Tera::default();
        tera.add_raw_template(
            "test_template",
            r#"
Loop:
{% for record in records %}{{loop.index}}.
String: {{ record.stringField }}
Int: {{ record.intField }}
Record: {{ record.recordField.fieldName }}
{% endfor %}
"#,
        )?;

        let mut context = Context::new();
        let mut records = Vec::new();
        let mut record = Record::new();
        add_field(
            record.fields_as_mut(),
            "stringField",
            model::value::Value::String("field-value".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "intField",
            model::value::Value::I32(73),
        );

        let mut sub_record = Record::new();
        add_field(
            sub_record.fields_as_mut(),
            "fieldName",
            model::value::Value::String("sub-record-value".to_string()),
        );
        add_field(
            record.fields_as_mut(),
            "recordField",
            model::value::Value::Record(sub_record),
        );

        records.push(crate::template::tera::record_to_tera_object(&record));

        context.insert("records", &records);

        let s = tera.render("test_template", &context)?;
        println!("{:?}", s);

        assert_eq!(
            "\nLoop:\n1.\nString: field-value\nInt: 73\nRecord: sub-record-value\n\n",
            s
        );

        Ok(())
    }
}
