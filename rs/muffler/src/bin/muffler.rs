//! CLI for the merge-sat crate

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Path to the input file.
    #[arg(short('i'), long)]
    inp_path: PathBuf,

    /// Path to the output directory.
    #[arg(short('o'), long)]
    out_dir: PathBuf,
}

fn main() -> Result<(), String> {
    let _args = Args::parse();

    Ok(())
}
