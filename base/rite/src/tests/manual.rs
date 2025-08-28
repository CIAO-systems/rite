use model::import::Importer;
use model::xml;
use model::plugin::Plugin;

static PLUGIN_PATH: &str = if cfg!(debug_assertions) {
    "../target/debug"
} else {
    "../target/release"
};
static IMPORT_PLUGIN_NAME: &str = "rite_csv";
static TEST_DATA: &str = "../data/test/testfile.txt";

fn load_importer() -> Result<Plugin, Box<dyn std::error::Error>> {
    Plugin::new(Some(PLUGIN_PATH), IMPORT_PLUGIN_NAME)
}

pub fn create_importer<'a>(
    importer_plugin: &'a mut Plugin,
) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
    // 1. load plugin
    // 2. Call creator function in plugin for an importer
    // 3. Initialize the importer
    // 3. box it and return it
    match importer_plugin.create_importer(None) {
        Ok(mut importer) => {
            let config = create_test_importer_config();
            let _ = importer.init(Some(config))?;
            Ok(importer)
        }
        Err(e) => Err(e),
    }
}

fn create_test_importer_config() -> xml::config::Configuration {
    let mut config = xml::config::Configuration::new();
    config.insert(String::from("file_name"), TEST_DATA.to_string());
    config
}

#[test]
#[ignore = "for manual testing"]
fn test_importer() -> Result<(), Box<dyn std::error::Error>> {
    model::helper::pwd();

    let mut importer_plugin = load_importer()?;
    let mut importer = create_importer(&mut importer_plugin)?;

    let config = create_test_importer_config();
    let _ = importer.init(Some(config))?;

    Ok(())
}

