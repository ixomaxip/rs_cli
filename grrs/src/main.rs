use std::io::{self, BufReader, BufRead};
use std::fs::File;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> io::Result<()> {
    let args = Cli::parse();
    let file = match File::open(&args.path) {
        Ok(f) => f,
        Err(error) => return Err(error.into())
    };
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
