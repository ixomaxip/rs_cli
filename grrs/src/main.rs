
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    println!("Hello, world!");
    let pattern = std::env::args().nth(1).expect("no pattern provided");
    let path = std::env::args().nth(2).expect("no path provided");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path)
    };
}
