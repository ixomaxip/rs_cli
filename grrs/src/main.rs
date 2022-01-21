use std::io::{BufReader, BufRead};
use std::fs::File;
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

#[derive(Debug)]
struct CustomError(String);
// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args.path.to_str()
        .with_context(|| "bad path")?;
    let file = File::open(path)
        .with_context(|| format!("could not read file '{}'", path))?;
    
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(error) => { println!("line error: {:?}", error); "".to_string() }
        };
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
