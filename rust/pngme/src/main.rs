mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use crate::args::{Cli, PngMeArgs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.subcommand {
        PngMeArgs::Encode(args) => commands::encode(args)?,
        PngMeArgs::Decode(args) => commands::decode(args)?,
        PngMeArgs::Remove(args) => commands::remove(args)?,
        PngMeArgs::Print(args) => commands::print_chunks(args)?,
    }
    Ok(())
}