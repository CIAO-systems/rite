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
At the very least, you will need the dependency to the RITE model library:
```toml
[dependencies]
model = { git = "https://github.com/CIAO-systems/rite-lib-model.git" }
```

# Implement an importer
An importer is responsible for reading data from a data source. 
## Dependencies
To be able to create implementations for the trait `Importer`, the library `import` must be added to the dependencies:
```toml
[dependencies]
import = { git = "https://github.com/CIAO-systems/rite-lib-import.git" }
```

To create an importer, the library needs to provide the function `create_importer` which has this signature:
```rust
/// This function creates an importer
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn import::Importer>, BoxedError>;
```
When your importer has multiple importers it can create, the parameter `name` tells the function which one to return:
```rust
/// This functions creates an importer for the given name
///
#[unsafe(no_mangle)]
pub fn create_importer(name: &str) -> Result<Box<dyn import::Importer>, BoxedError> {
    match name {
        "fancy_importer" => Ok(Box::new(FancyImporter::new())),
        "yafi" => Ok(Box::new(YetAnotherFancyImporter::new())),
        _ => Err(format!("Unknown importer '{name}'").into()),
    }
}
```
# Implement an exporter
# Implement a transformer