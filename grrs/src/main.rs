use std::io::{BufReader, BufRead};
use std::fs::File;
use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};
use env_logger;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    for (idx, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(error) => { warn!("line error: {:?}", error); "".to_string() }
        };
        if line.contains(pattern) {
            writeln!(writer, "{}\t{}", idx, line);
        }
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let path = args.path.to_str()
        .with_context(|| "bad path")?;
    
    let file = File::open(path)
        .with_context(|| format!("could not read file '{}'", path))?;
    
    info!("read file {}", path);
    
    let reader = BufReader::new(file);
    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}
