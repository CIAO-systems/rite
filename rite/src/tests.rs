use export::Exporter;
use import::Importer;
use plugin::Plugin;
use std::{env, io::Read};
use transform::{
    builtin::string::{StringFieldConversion, StringFieldConverter},
    Transformer,
};

static PLUGIN_PATH: &str = "../target/debug";
static IMPORT_PLUGIN_NAME: &str = "example_import";
static EXPORT_PLUGIN_NAME: &str = "example_export";

fn load_importer() -> Result<Plugin, Box<dyn std::error::Error>> {
    Plugin::new(PLUGIN_PATH, IMPORT_PLUGIN_NAME)
}

fn load_exporter() -> Result<Plugin, Box<dyn std::error::Error>> {
    Plugin::new(PLUGIN_PATH, EXPORT_PLUGIN_NAME)
}

fn create_transformer() -> Result<Box<dyn Transformer>, Box<dyn std::error::Error>> {
    let transformer = StringFieldConverter::new(StringFieldConversion::UpperCase);
    Ok(Box::new(transformer))
}

fn create_exporter<'a>(
    exporter_plugin: &'a mut Plugin,
    exporter_name: &str,
) -> Result<&'a mut Box<dyn Exporter>, Box<dyn std::error::Error>> {
    // 1. load plugin
    // 2. Call creator function in plugin for an exporter
    // 3. box it and return it
    match exporter_plugin.create_exporter(exporter_name) {
        Ok(exporter) => {
            let _ = exporter.init();
            Ok(exporter)
        }
        Err(e) => Err(e),
    }
}

pub fn create_importer<'a>(
    importer_plugin: &'a mut Plugin,
    importer_name: &str,
) -> Result<&'a mut Box<dyn Importer>, Box<dyn std::error::Error>> {
    // 1. load plugin
    // 2. Call creator function in plugin for an importer
    // 3. box it and return it
    match importer_plugin.create_importer(importer_name) {
        Ok(importer) => {
            let _ = importer.init();
            Ok(importer)
        }
        Err(e) => Err(e),
    }
}

#[test]
fn test_big_picture() {
    if let Ok(cwd) = env::current_dir() {
        println!("{}", cwd.display());
    }

    // Redirect stdout
    let buf = gag::BufferRedirect::stdout().unwrap();

    match load_importer() {
        Ok(mut importer_plugin) => match create_importer(&mut importer_plugin, "text") {
            Ok(importer) => match load_exporter() {
                Ok(mut exporter_plugin) => match create_exporter(&mut exporter_plugin, "console") {
                    Ok(exporter) => match create_transformer() {
                        Ok(transformer) => {
                            check_all(importer, transformer, exporter, buf);
                        }
                        Err(e) => panic!("create_transformer: {e}"),
                    },
                    Err(e) => panic!("create_exporter: {e}"),
                },
                Err(e) => panic!("load_exporter: {e}"),
            },
            Err(e) => panic!("create_importer: {e}"),
        },
        Err(e) => panic!("load_importer: {e}"),
    }
}

fn check_all(
    importer: &mut Box<dyn Importer>,
    transformer: Box<dyn Transformer>,
    exporter: &mut Box<dyn Exporter>,
    mut buf: gag::BufferRedirect,
) {
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
