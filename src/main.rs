// src/main.rs
/*
 * Main executable for ArtificialEaselSolutionsUltra
 */

use clap::Parser;
use artificialeaselsolutionsultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialEaselSolutionsUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
