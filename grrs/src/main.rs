use std::io::{self, BufReader, BufRead};
use std::fs::File;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

#[derive(Debug)]
struct CustomError(String);
// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let path = args.path.to_str().unwrap();
    let file = File::open(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(error) => { println!("line error: {:?}", error); "".to_string() }
        };
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
    }

    Ok(())
}
