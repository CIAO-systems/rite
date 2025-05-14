use crate::transformer::mapper::config::mapper::Mapper;
use crate::transformer::mapper::config::{Field, Name, Type};
use std::io::Write;

#[test]
fn test_mapper_new_and_get() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary XML file for testing
    let xml_content = r#"
        <Mapper>
          <field>
            <name source="field1" target="mapped_field1" />
            <type source="string" target="string" />
          </field>
          <field>
            <name source="field_two" target="the_second_field" />
            <type source="string" target="string" />
          </field>
        </Mapper>
    "#;
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(xml_content.as_bytes())?;
    let file_path = temp_file.path().to_str().unwrap();

    // Create a Mapper instance from the temporary file
    let mapper = Mapper::new(file_path)?;

    // Test the 'get' method
    let field1 = mapper.get("field1".to_string());
    assert!(field1.is_some());
    assert_eq!(
        field1.unwrap(),
        Field {
            name: Name {
                source: "field1".to_string(),
                target: "mapped_field1".to_string(),
            },
            field_type: Type {
                source: "string".to_string(),
                target: "string".to_string()
            },
            patterns: None,
            values: None
        }
    );

    let field_two = mapper.get("field_two".to_string());
    assert!(field_two.is_some());
    assert_eq!(
        field_two.unwrap(),
        Field {
            name: Name {
                source: "field_two".to_string(),
                target: "the_second_field".to_string(),
            },
            field_type: Type {
                source: "string".to_string(),
                target: "string".to_string()
            },
            patterns: None,
            values: None
        }
    );

    let non_existent_field = mapper.get("non_existent".to_string());
    assert!(non_existent_field.is_none());

    Ok(())
}

#[test]
fn test_mapper_new_file_not_found() {
    let result = Mapper::new("non_existent_file.xml");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "No such file or directory (os error 2)"
    ); // Adjust the error message based on your OS
}

#[test]
fn test_mapper_new_invalid_xml() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary XML file with invalid content
    let xml_content = r#"
        <Mapper>
          <field>
            <name>
              <source>field1</source>
              <target>mapped_field1</target>
            </name>
            <data_type>String</data_type>
          </invalid_tag>
        </Mapper>
    "#;
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(xml_content.as_bytes())?;
    let file_path = temp_file.path().to_str().unwrap();

    // Try to create a Mapper instance from the invalid XML file
    let result = Mapper::new(file_path);
    assert!(result.is_err());
    let error = result.as_ref().unwrap_err();
    println!("{}", error.to_string());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("missing field `@source`")); 

    Ok(())
}
