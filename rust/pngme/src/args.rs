use std::path::PathBuf;

use clap::{Parser, ArgMatches, Command, Args};
use clap::error::{Error, ErrorKind};

#[derive(Parser, Debug)]
pub struct Cli {
   #[clap(subcommand)]
   pub subcommand: PngMeArgs, 
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    // Path
    #[clap(value_parser)]
    pub path: PathBuf,
    // Chunk Type
    #[clap(value_parser)]
    pub chunk_type: String,
    // Message
    #[clap(value_parser)]
    pub msg: String,
    // Output (optional)
    #[clap(value_parser)]
    pub output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    // Path
    #[clap(value_parser)]
    pub path: PathBuf,
    // Chunk Type
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    // Path
    #[clap(value_parser)]
    pub path: PathBuf,
    // Chunk Type
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    // Path
    #[clap(value_parser)]
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

impl clap::FromArgMatches for PngMeArgs {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("encode", args)) => Ok(Self::Encode(EncodeArgs::from_arg_matches(args)?)),
            Some(("decode", args)) => Ok(Self::Decode(DecodeArgs::from_arg_matches(args)?)),
            Some(("remove", args)) => Ok(Self::Remove(RemoveArgs::from_arg_matches(args)?)),
            Some(("print", args)) => Ok(Self::Print(PrintArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::UnrecognizedSubcommand,
                "Valid subcommands are `encode`, `decode`, `remove`, and `print`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `encode`, `decode`, `remove`, and `print`",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("encode", args)) => *self = Self::Encode(EncodeArgs::from_arg_matches(args)?),
            Some(("decode", args)) => *self = Self::Decode(DecodeArgs::from_arg_matches(args)?),
            Some(("remove", args)) => *self = Self::Remove(RemoveArgs::from_arg_matches(args)?),
            Some(("print", args)) => *self = Self::Print(PrintArgs::from_arg_matches(args)?),
            Some((_,_)) => {
                return Err(Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    "Valid subcommands are `encode`, `decode`, `remove`, and `print`"
                ))
            },
            None => (),
        };

        Ok(())
    }
}

impl clap::Subcommand for PngMeArgs {
    fn augment_subcommands(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(EncodeArgs::augment_args(Command::new("encode")))
            .subcommand(DecodeArgs::augment_args(Command::new("decode")))
            .subcommand(RemoveArgs::augment_args(Command::new("remove")))
            .subcommand(PrintArgs::augment_args(Command::new("print")))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(EncodeArgs::augment_args(Command::new("encode")))
            .subcommand(DecodeArgs::augment_args(Command::new("decode")))
            .subcommand(RemoveArgs::augment_args(Command::new("remove")))
            .subcommand(PrintArgs::augment_args(Command::new("print")))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "encode" | "decode" | "remove" | "print")
    }
}
