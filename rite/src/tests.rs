use export::{builtin::console::ConsoleExporter, Exporter};
use import::Importer;
use plugin::Plugin;
use std::{env, io::Read};
use transform::{
    builtin::string::{StringFieldConversion, StringFieldConverter},
    Transformer,
};

struct TestData {
    importer_plugin: Plugin,
}

static IMPORT_PLUGIN_PATH: &str = "../target/debug";
static IMPORT_PLUGIN_NAME: &str = "example_import";

impl TestData {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        match Plugin::new(IMPORT_PLUGIN_PATH, IMPORT_PLUGIN_NAME) {
            Ok(plugin) => Ok(TestData {
                importer_plugin: plugin,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn create_transformer(&self) -> StringFieldConverter {
        StringFieldConverter::new(StringFieldConversion::UpperCase)
    }

    pub fn create_exporter(&self) -> Result<ConsoleExporter, Box<dyn std::error::Error>> {
        let mut exporter = ConsoleExporter::new();
        exporter.init()?;
        Ok(exporter)
    }

    pub fn create_importer(
        &mut self,
        importer_name: &str,
    ) -> Result<&mut Box<dyn Importer>, Box<dyn std::error::Error>> {
        // 1. load plugin
        // 2. Call creator function in plugin for an importer
        // 3. box it and return it
        match self.importer_plugin.create_importer(importer_name) {
            Ok(importer) => {
                let _ = importer.init();
                Ok(importer)
            }
            Err(e) => Err(e),
        }
    }
}

#[test]
fn test_big_picture() {
    if let Ok(cwd) = env::current_dir() {
        println!("{}", cwd.display());
    }

    // Redirect stdout
    let mut buf = gag::BufferRedirect::stdout().unwrap();

    match TestData::new() {
        Ok(mut test_data) => {
            let transformer = test_data.create_transformer();
            let exporter = test_data.create_exporter();
            let importer = test_data.create_importer("text");

            match importer {
                Ok(importer) => {
                    match exporter {
                        Ok(mut exporter) => {
                            let _ = importer.read(&mut |record| {
                                // transform
                                match transformer.process(&record) {
                                    Ok(transformed) => {
                                        // export
                                        if let Err(e) = exporter.write(&transformed) {
                                            panic!("{e}");
                                        }
                                    }
                                    Err(e) => panic!("{e}"),
                                }
                            });

                            // Read the output into a string
                            let mut output = String::new();
                            let _ = buf.read_to_string(&mut output);

                            assert_eq!(
                                "line=LINE1,index=1\nline=LINE2,index=2\nline=LINE3,index=3\nline=,index=4\nline=LINE5,index=5\n", 
                                output);
                        }
                        Err(e) => panic!("{e}"),
                    }
                }
                Err(e) => panic!("{e}"),
            }
        }
        Err(e) => panic!("{e}"),
    }
}
