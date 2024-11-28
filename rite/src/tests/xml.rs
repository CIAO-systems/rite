use std::collections::HashMap;

use model::xml::{self, file::create_rite};

static EXAMPLE_XML: &str = "../data/example.xml";

#[test]
fn test_with_xml() -> Result<(), Box<dyn std::error::Error>> {
    let rite = create_rite(EXAMPLE_XML)?;

    // load all plugins
    let plugins: HashMap<&str, &xml::Plugin> = rite
        .plugins
        .plugins
        .iter()
        .map(|p| (p.id.as_str(), p))
        .collect();

    // let _ = plugins
    //     .iter()
    //     .for_each(|(key, value)| println!("{key}={}", value.name));

    // println!("{:#?}", plugins);

    for process in rite.processes.processes {
        // Import data using the importer
        if let Some(plugin_desc) = plugins.get(&process.importer.plugin.as_str()) {
            println!("Importer plugin: {:#?}", plugin_desc);

            let mut importer_plugin = plugin::Plugin::new(&plugin_desc.path, &plugin_desc.name)?;

            let importer = importer_plugin.create_importer(&process.importer.name)?;

            let config = process.importer.configuration;
            let _ = importer.init(config)?;

            let _ = importer.read(&mut |record| {
                println!("{:#?}", record);
                // FIXME complete this

                // transform
                // match transformer.process(&record) {
                //     Ok(transformed) => {
                //         // export
                //         if let Err(e) = exporter.write(&transformed) {
                //             panic!("{e}");
                //         }
                //     }
                //     Err(e) => panic!("{e}"),
                // }
            });
        }
    }
    Ok(())
}
