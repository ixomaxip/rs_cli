use std::io::BufReader;
use std::fs::File;
use clap::Parser;
use anyhow::{Context, Result};

use grrs::helpers::finder::find_matches;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let path = args.path.to_str()
        .with_context(|| "bad path")?;
    
    let file = File::open(path)
        .with_context(|| format!("could not read file '{}'", path))?;
    
    let reader = BufReader::new(file);
    find_matches(reader, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
