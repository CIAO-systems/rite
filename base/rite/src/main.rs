use clap::Parser;
use dotenv::dotenv;
use log::info;
use rite::processor;

/// Struct for the command line options for the replay binary
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// XML File location
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    log4rs::init_file("log4rs.yaml", Default::default())?;
    info!("Rust Import/Transform/Export");

    let cli = Cli::parse();
    if let Some(file) = cli.file {
        let mut rp = processor::rite::Rite::new(&file)?;
        match rp.init() {
            Ok(_) => match rp.process() {
                Ok(_) => log::info!("Successfully processed"),
                Err(e) => log::error!("Error processing: {}", e),
            },
            Err(e) => log::error!("Error initializing: {}", e),
        }
    } else {
        log::error!("No XML file given. Try with -f <filename> or --file=<filename>");
    }

    Ok(())
}

#[cfg(test)]
mod tests;
