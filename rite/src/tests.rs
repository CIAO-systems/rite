use export::Exporter;
use import::Importer;
use plugin::Plugin;
use std::io::Read;
use transform::Transformer;

static PLUGIN_PATH: &str = "../target/debug";
static IMPORT_PLUGIN_NAME: &str = "example_import";
static EXPORT_PLUGIN_NAME: &str = "example_export";
static TRANSFORM_PLUGIN_NAME: &str = "example_transform";

fn load_importer() -> Result<Plugin, Box<dyn std::error::Error>> {
    Plugin::new(PLUGIN_PATH, IMPORT_PLUGIN_NAME)
}

fn load_exporter() -> Result<Plugin, Box<dyn std::error::Error>> {
    Plugin::new(PLUGIN_PATH, EXPORT_PLUGIN_NAME)
}

fn load_transformer() -> Result<Plugin, Box<dyn std::error::Error>> {
    Plugin::new(PLUGIN_PATH, TRANSFORM_PLUGIN_NAME)
}

fn create_transformer<'a>(
    transformer_plugin: &'a mut Plugin,
    transformer_name: &str,
) -> Result<&'a mut Box<dyn Transformer>, Box<dyn std::error::Error>> {
    // 1. load plugin
    // 2. Call creator function in plugin for a transformer
    // 3. box it and return it
    match transformer_plugin.create_transformer(transformer_name) {
        Ok(transformer) => Ok(transformer),
        Err(e) => Err(e),
    }
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

fn check_all(
    expected: &str,
    importer: &mut Box<dyn Importer>,
    transformer: &mut Box<dyn Transformer>,
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

    assert_eq!(expected, output);
}


#[test]
fn test_big_picture() -> Result<(), Box<dyn std::error::Error>> {
    let _ = test_lowercase()?;
    let _ = test_uppercase()?;

    Ok(())
}

fn test_uppercase() -> Result<(), Box<dyn std::error::Error>> {
    // Redirect stdout
    let buf = gag::BufferRedirect::stdout().unwrap();

    let mut importer_plugin = load_importer()?;
    let mut exporter_plugin = load_exporter()?;
    let mut transformer_plugin = load_transformer()?;

    let importer = create_importer(&mut importer_plugin, "text")?;
    let exporter = create_exporter(&mut exporter_plugin, "console")?;
    let transformer = create_transformer(&mut transformer_plugin, "uppercase")?;

    check_all(
        "line=LINE1,index=1\nline=LINE2,index=2\nline=LINE3,index=3\nline=,index=4\nline=LINE5,index=5\n",
        importer, transformer, exporter, buf);

    Ok(())
}

fn test_lowercase() -> Result<(), Box<dyn std::error::Error>> {
    // Redirect stdout
    let buf = gag::BufferRedirect::stdout().unwrap();

    let mut importer_plugin = load_importer()?;
    let mut exporter_plugin = load_exporter()?;
    let mut transformer_plugin = load_transformer()?;

    let importer = create_importer(&mut importer_plugin, "text")?;
    let exporter = create_exporter(&mut exporter_plugin, "console")?;
    let transformer = create_transformer(&mut transformer_plugin, "lowercase")?;

    check_all(
        "line=line1,index=1\nline=line2,index=2\nline=line3,index=3\nline=,index=4\nline=line5,index=5\n",
        importer, transformer, exporter, buf);

    Ok(())
}
