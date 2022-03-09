use clap::Parser;
use std::process;

mod filter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Will read from STDIN if omitted
    file: Option<String>,

    /// Disable highlighting of replacements
    #[clap(long)]
    no_highlight: bool,

    /// Format dates in UTC instead of the system's local time
    #[clap(long)]
    utc: bool,
}

fn main() {
    let args = Args::parse();

    match &args.file {
        Some(path) => {
            if filter::filter_file(path, &args).is_err() {
                eprintln!("Could not read file at \"{}\"", path);
                process::exit(1);
            }
        }
        None => filter::filter_stdin(&args),
    }
}
