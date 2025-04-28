use std::collections::HashMap;

use crate::xml::file::create_rite;

static EXAMPLE_XML: &str = "../data/test/test-example.xml";
static EXAMPLE_2_XML: &str = "../data/test/test-example-2.xml";

#[test]
#[ignore]
fn test_example_xml() -> Result<(), Box<dyn std::error::Error>> {
    let rite = create_rite(EXAMPLE_XML, &HashMap::new())?;
    println!("{:#?}", rite);

    // Add some basic assertions to verify the parsing
    assert!(
        !rite.plugins.plugins.is_empty(),
        "Plugins should not be empty"
    );
    assert_eq!(rite.plugins.plugins.len(), 3, "Expected 3 plugins");

    assert!(
        !rite.processes.processes.is_empty(),
        "Processes should not be empty"
    );
    assert_eq!(rite.processes.processes.len(), 1, "Expected 1 process");

    // Verify specific plugin details
    let import_plugin = &rite.plugins.plugins[0];
    assert_eq!(import_plugin.id, "import_plugin");
    assert_eq!(import_plugin.path, Some("../target/debug".to_string()));
    assert_eq!(import_plugin.name, "example_import");

    // Verify process details
    let process = &rite.processes.processes[0];
    assert_eq!(process.id, "text-uppercase-console");

    // Check importer details
    assert_eq!(process.importer.plugin, "import_plugin");
    assert_eq!(process.importer.name, Some("text".to_string()));

    // Check importer configuration
    if let Some(config) = &process.importer.configuration {
        assert_eq!(config.len(), 1);
        assert_eq!(
            config.get("file_name"),
            Some("../data/test/testfile.txt".to_string())
        );
    } else {
        panic!("Importer configuration should exist");
    }

    // Print parsed data for manual inspection
    println!("Parsed Rite XML: {:?}", rite);
    Ok(())
}

#[test]
#[ignore]
fn test_example_2_xml() -> Result<(), Box<dyn std::error::Error>> {
    let rite = create_rite(EXAMPLE_2_XML, &HashMap::new())?;

    // Add some basic assertions to verify the parsing
    assert!(
        !rite.plugins.plugins.is_empty(),
        "Plugins should not be empty"
    );
    assert_eq!(rite.plugins.plugins.len(), 2, "Expected 3 plugins");

    assert!(
        !rite.processes.processes.is_empty(),
        "Processes should not be empty"
    );
    assert_eq!(rite.processes.processes.len(), 1, "Expected 1 process");

    // Verify specific plugin details
    let import_plugin = &rite.plugins.plugins[0];
    assert_eq!(import_plugin.id, "import_plugin");
    assert_eq!(import_plugin.path, None);
    assert_eq!(import_plugin.name, "example_import");

    // Verify process details
    let process = &rite.processes.processes[0];
    assert_eq!(process.id, "text-uppercase-console");

    // Check importer details
    assert_eq!(process.importer.plugin, "import_plugin");
    assert_eq!(process.importer.name, Some("text".to_string()));

    // Check importer configuration
    if let Some(config) = &process.importer.configuration {
        assert_eq!(config.len(), 0);
        assert_eq!(config.get("file_name"), None);
    } else {
        panic!("Importer configuration should exist");
    }

    // Print parsed data for manual inspection
    println!("Parsed Rite XML: {:?}", rite);
    Ok(())
}
