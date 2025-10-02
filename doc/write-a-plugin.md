# Setup a cargo project 
A RITE plugin is a [dynamic rust library](https://doc.rust-lang.org/reference/linkage.html#r-link.dylib).
To create a new Cargo project for a dynamic rust library, enter the following command:
```bash
$> cargo new <plugin_name> --lib
```

In your `Cargo.toml` you need the section
```toml
[lib]
name = "<plugin_name>"
crate-type = ["dylib"]
```
## Dependencies
You will need the dependency to the RITE model library:
```toml
[dependencies]
model = { git = "https://github.com/CIAO-systems/rite-lib-model.git", branch = "main" }
```

# Implement an importer
An importer is responsible for reading data from a data source. 

To create an importer, the library needs to provide the function `create_importer` which has this signature:
```rust
/// This function creates an importer
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn model::import::Importer>, BoxedError>;
```
When your plugin has multiple importers it can create, the parameter `name` tells the function which one to return:
```rust
/// This function creates an importer for the given name
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn model::import::Importer>, BoxedError> {
    match name {
        "fancy_importer" => Ok(Box::new(FancyImporter::new())),
        "yafi" => Ok(Box::new(YetAnotherFancyImporter::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}
```
# Implement an exporter
An exporter is responsible for writing data to a data sink.

To create an exporter, the library needs to provide the function `create_exporter` which has this signature:
```rust
/// Plugin entry function to create an instance of an [Exporter]
/// # Arguments
/// * `name` - When the plugin supports multiple exporters, the `name` is used
///     to determined, what exporter to return
#[unsafe(no_mangle)]
pub fn create_exporter(name: &str) -> Result<Box<dyn model::export::Exporter>, BoxedError>;
```
When your plugin has multiple exporters it can create, the parameter `name` tells the function which one to return:
```rust
/// Plugin entry function to create an instance of an [Exporter]
/// # Arguments
/// * `name` - When the plugin supports multiple exporters, the `name` is used
///     to determined, what exporter to return
#[unsafe(no_mangle)]
pub fn create_exporter(name: &str) -> Result<Box<dyn model::export::Exporter>, BoxedError> {
    match name {
        "fancy_exporter" => Ok(Box::new(FancyExporter::new())),
        "yafe" => Ok(Box::new(YetAnotherFancyExporter::new())),
        _ => Err(format!("Unknown exporter '{name}'").into()),
    }
}
```
# Implement a transformer
A transformer is responsible for changing/adding or removing of fields in every record read by the importer 
before it gets written by an exporter

To create a transformer, the library needs to provide the function `create_transformer` which has this signature:
```rust
/// Plugin entry function to create an instance of an [Transformer]
#[unsafe(no_mangle)]
pub fn create_transformer(
    name: &str,
) -> Result<Box<dyn model::transform::Transformer>, BoxedError>;
```
When your plugin has multiple transformers it can create, the parameter `name` tells the function which one to return:
```rust
/// Plugin entry function to create an instance of an [Transformer]
#[unsafe(no_mangle)]
pub fn create_transformer(
    name: &str,
) -> Result<Box<dyn model::transform::Transformer>, BoxedError> {
    match name {
        "fancy_transformer" => Ok(Box::new(FancyTransformer::new())),
        "yaft" => Ok(Box::new(YetAnotherFancyTransformer::new())),
        _ => Err(format!("Unknown transformer '{name}'").into()),
    }
}
```