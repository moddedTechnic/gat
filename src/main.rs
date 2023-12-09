mod hash_object;
mod initialise;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::{hash_object::*, initialise::*};

#[derive(Debug, Error)]
enum GatError {
    #[error("initialsation failed")]
    InitialisationError(#[from] InitialisationError),
    #[error("couldn't hash the object")]
    HashObjectError(#[from] HashObjectError),
}

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "gat")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    HashObject { path: PathBuf },
}

fn main() -> Result<(), GatError> {
    let args = Cli::parse();

    match args.command {
        Commands::Init => init()?,
        Commands::HashObject { path } => hash_object(path)?,
    };
    Ok(())
}
