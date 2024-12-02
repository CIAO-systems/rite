use log::info;

mod processor;
static EXAMPLE_XML: &str = "data/example.xml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    helper::pwd();
    log4rs::init_file("log4rs.yaml", Default::default())?;

    info!("Rust Import/Transform/Export");

    let rp = processor::Rite::new(EXAMPLE_XML)?;
    rp.process()?;

    Ok(())
}

#[cfg(test)]
mod tests;
