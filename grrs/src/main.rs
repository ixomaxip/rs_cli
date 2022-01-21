use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
    }
    
    Ok(())
}
