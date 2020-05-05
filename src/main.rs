use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
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
