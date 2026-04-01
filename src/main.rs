mod args;
mod error;
mod format;
mod report;
mod png;
mod jpeg;
mod dng;

use args::{Args, Command};
use clap::Parser;
use std::fs;

fn main() {
    let args = Args::parse();

    match args.command {
    Command::Inspect { path } => {
        let data = fs::read(&path).expect("Failed to read file");
        let report = format::inspect(&data).expect("Inspection failed");
        // Use the default Display implementation (shows everything)
        println!("{}", report); 
    }
    Command::Metadata { path } => {
        let data = fs::read(&path).expect("Failed to read file");
        let report = format::inspect(&data).expect("Inspection failed");
        // Use our new specific method (hides binary hex info)
        report.display_metadata_only(); 
    }
}
}
