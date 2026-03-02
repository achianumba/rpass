use clap::Parser;

/// Invokes rpass command line interface.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    rpass::cli::Cli::parse().run()?;
    Ok(())
}
