use std::path::PathBuf;

use clap::Parser;

/// A tiny whilelist forward program.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set config file location, default is current folder.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}
