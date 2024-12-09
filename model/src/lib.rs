pub mod field;
pub mod record;
pub mod value;
pub mod xml;

pub trait Initializable {
    /// Initializes the object
    ///
    /// `config` should be stored in the object, to make sure, all
    /// configuration values live as long the instance
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
