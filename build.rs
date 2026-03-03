use std::{env, fs::write, path::PathBuf};

fn main() {
    let dir_str = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to read the CARGO_MANIFEST_DIR environment variable.");
    let dir = PathBuf::from(dir_str);
    let docs_dir = dir.join("docs");
    
    generate_long_help_header(docs_dir.join("cli-header.md"));
}

/// Writes the first line of the binary crate's docs & cli help message to a static file
/// before building the crate
fn generate_long_help_header(header_file: PathBuf) {
    write(&header_file, generate_cli_header())
        .expect(format!("Failed to write CLI header to {}", header_file.display()).as_str());
}

/// Provides uniform header text in the binary crate's docs and the command line.
fn generate_cli_header() -> &'static str {
    concat!(
        env!("CARGO_PKG_NAME"),
        " ",
        env!("CARGO_PKG_VERSION"),
        " by ",
        env!("CARGO_PKG_AUTHORS")
    )
}
