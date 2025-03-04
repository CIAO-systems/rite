use export::{Exporter, Signal};
use model::{field::add_field, record::Record, xml::config::Configuration, Initializable};
use tera::{Context, Tera};
use uuid::Uuid;

use super::{TemplateExporter, CFG_OUTPUT_FILE, CFG_TEMPLATE_FILE};

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

#[test]
fn test_exporter() -> Result<(), model::BoxedError> {
    let mut exporter = TemplateExporter::new();
    let mut config = Configuration::new();
    config.insert_str(
        CFG_TEMPLATE_FILE,
        "../../data/common/templates/test_template.txt",
    );
    const TMP_FILE: &str = "/tmp/template_output.txt";
    config.insert_str(CFG_OUTPUT_FILE, TMP_FILE);

    exporter.init(Some(config))?;
    exporter.event(Signal::Start)?;

    let mut record = Record::new();
    let uuid = Uuid::new_v4();
    add_field(
        record.fields_as_mut(),
        "id",
        model::value::Value::String(uuid.to_string()),
    );
    exporter.write(&record)?;
    exporter.event(Signal::End)?;

    let content = std::fs::read_to_string(TMP_FILE)?;
    assert_eq!(format!("1. {}", uuid.to_string()), content);

    Ok(())
}

#[test]
fn test_exporter_with_record() -> Result<(), model::BoxedError> {
    let mut exporter = TemplateExporter::new();
    let mut config = Configuration::new();
    config.insert_str(
        CFG_TEMPLATE_FILE,
        "../../data/common/templates/test_template_with_record.txt",
    );
    const TMP_FILE: &str = "/tmp/template_with_record_output.txt";
    config.insert_str(CFG_OUTPUT_FILE, TMP_FILE);

    exporter.init(Some(config))?;
    exporter.event(Signal::Start)?;

    let mut record = Record::new();
    let uuid = Uuid::new_v4();
    add_field(
        record.fields_as_mut(),
        "id",
        model::value::Value::String(uuid.to_string()),
    );
    let mut sub_record = Record::new();
    add_field(
        sub_record.fields_as_mut(),
        "number",
        model::value::Value::I32(73),
    );
    add_field(
        record.fields_as_mut(),
        "sub",
        model::value::Value::Record(sub_record),
    );

    exporter.write(&record)?;
    exporter.event(Signal::End)?;

    let content = std::fs::read_to_string(TMP_FILE)?;
    assert_eq!(format!("\n{} = 73\n", uuid.to_string()), content);

    Ok(())
}

#[test]
fn test_exporter_with_collection() -> Result<(), model::BoxedError> {
    let mut exporter = TemplateExporter::new();
    let mut config = Configuration::new();
    config.insert_str(
        CFG_TEMPLATE_FILE,
        "../../data/common/templates/test_template_with_collection.txt",
    );
    const TMP_FILE: &str = "/tmp/template_with_collection_output.txt";
    config.insert_str(CFG_OUTPUT_FILE, TMP_FILE);

    exporter.init(Some(config))?;
    exporter.event(Signal::Start)?;

    let mut record = Record::new();
    add_field(
        record.fields_as_mut(),
        "collection",
        model::value::Value::Collection(vec![
            model::value::Value::I32(42),
            model::value::Value::F64(73.42),
            model::value::Value::String("It is not a number!".to_string()),
        ]),
    );

    exporter.write(&record)?;
    exporter.event(Signal::End)?;

    let content = std::fs::read_to_string(TMP_FILE)?;
    assert_eq!("\n\n42\n\n73.42\n\nIt is not a number!\n\n", content);

    Ok(())
}
