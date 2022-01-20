use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
