use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let f = File::open(args.path).with_context(|_| format!("could not open file"))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(text) if text.starts_with(&args.pattern) => {
                println!("{}", text);
            }
            Ok(_) => (), // match line that not having the given pattern
            Err(_) => (),
        }
    }
    Ok(())
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
