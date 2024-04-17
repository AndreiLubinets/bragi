use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    #[arg(short, long)]
    pub debug: bool,
}
