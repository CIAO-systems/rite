use export::Exporter;
use import::Importer;
use libloading::{Library, Symbol};
use transform::Transformer;

const CREATE_EXPORTER: &[u8] = b"create_exporter";
const CREATE_IMPORTER: &[u8] = b"create_importer";
const CREATE_TRANSFORMER: &[u8] = b"create_transformer";

pub type ImporterCreator =
    unsafe fn(name: Option<&str>) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>>;
pub type ExporterCreator =
    unsafe fn(name: Option<&str>) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>>;
pub type TransformerCreator =
    unsafe fn(name: Option<&str>) -> Result<Box<dyn Transformer>, Box<dyn std::error::Error>>;

pub struct Plugin {
    // this must be last, so it get dropped last
    _lib: Library,
}

impl Plugin {
    /// Creates an instance of the [Plugin] struct.
    ///
    /// `path` is the fully qualified directory name, where the dynamic library is
    /// located
    /// `name` is a platorm agnostic name of the library (without prefix `lib` and
    /// without extension)
    pub fn new(path: Option<&str>, name: &str) -> Result<Plugin, Box<dyn std::error::Error>> {
        // check the OS we're running on
        let os_lib_name = match std::env::consts::OS {
            "linux" => format!("lib{name}.so"),
            "macos" => format!("lib{name}.dylib"),
            "windows" => format!("{name}.dll"),
            _ => return Err("Unsupported operating system".into()),
        };

        // Only if a path is given, prefix the file_name with it
        // Without path, library will be loaded using the system library search
        // path
        let lib_path = if let Some(path) = path {
            format!("{path}/{os_lib_name}")
        } else {
            format!("{os_lib_name}")
        };

        let _lib = unsafe {
            log::debug!("Loading {}", lib_path);
            Library::new(lib_path)?
        };

        Ok(Self { _lib })
    }

    pub fn create_importer(
        &self,
        name: Option<&str>,
    ) -> Result<Box<dyn Importer>, Box<dyn std::error::Error>> {
        let creator: Symbol<ImporterCreator> = unsafe { self._lib.get(CREATE_IMPORTER)? };
        unsafe { creator(name) }
    }

    pub fn create_exporter(
        &self,
        name: Option<&str>,
    ) -> Result<Box<dyn Exporter>, Box<dyn std::error::Error>> {
        let creator: Symbol<ExporterCreator> = unsafe { self._lib.get(CREATE_EXPORTER)? };
        unsafe { creator(name) }
    }

    pub fn create_transformer(
        &self,
        name: Option<&str>,
    ) -> Result<Box<dyn Transformer>, Box<dyn std::error::Error>> {
        let creator: Symbol<TransformerCreator> = unsafe { self._lib.get(CREATE_TRANSFORMER)? };
        unsafe { creator(name) }
    }
}

mod tests {

    #[test]
    fn exp() {
        struct Data {
            value: std::cell::RefCell<Option<i32>>,
        }

        impl Data {
            fn get(&self, new_value: i32) -> i32 {
                let mut value = self.value.borrow_mut();

                if value.is_none() {
                    *value = Some(new_value);
                }
                value.unwrap()
            }
        }

        let data = Data {
            value: std::cell::RefCell::new(None),
        };

        assert_eq!(42, data.get(42));
        assert_eq!(42, data.get(73));
    }
}
