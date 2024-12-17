pub mod field;
pub mod record;
pub mod value;
pub mod xml;

/// Struct that implement this trait can be initialized with a
///  [xml::config::Configuration]
///
pub trait Initializable {
    /// Initializes the object
    ///
    /// `config` should be stored in the object, to make sure, all
    /// configuration values live as long the instance
    ///
    /// # Arguments
    /// * `config` - An optional [xml::config::Configuration]. The implementing 
    ///              object should take ownership of the config object
    ///
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
