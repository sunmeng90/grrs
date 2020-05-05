use std::fs;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let content = fs::read_to_string(&args.path).expect("could not read the file");
    for line in content.lines() {
        if line.starts_with(&args.pattern) {
            println!("{}", line);
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to file to read
    // PathBuf is like a String but for file system paths that works cross-platform.
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
