mod processor;
static EXAMPLE_XML: &str = "data/example.xml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust Import/Transform/Export");
    helper::pwd();

    let rp = processor::Rite::new(EXAMPLE_XML)?;
    rp.process()?;

    Ok(())
}

#[cfg(test)]
mod tests;
