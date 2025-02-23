use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct FunctionalSimArgs {
    /// Output log file
    #[arg(long, default_value = "LOG.txt")]
    pub log: PathBuf,

    /// Path to target binary
    #[arg(long)]
    pub bin: PathBuf
}
