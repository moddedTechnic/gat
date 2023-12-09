use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use thiserror::Error;

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

#[derive(Error, Debug)]
enum InitialisationError {
    #[error("an io error occurred")]
    IoError(#[from] std::io::Error),
    #[error("a repository already exists")]
    ExistingRepo,
}

fn init() -> Result<(), InitialisationError> {
    let repo_dir = PathBuf::from("../gat-test");
    let git_dir = repo_dir.join(".git");

    if std::path::Path::new(&git_dir).exists() {
        return Err(InitialisationError::ExistingRepo);
    }

    println!("Initialising new repository in {repo_dir:?}");
    fs::create_dir_all(&git_dir)?;

    let mut file = File::create(git_dir.join("HEAD"))?;
    file.write_all(b"ref: refs/head/main\n")?;

    let mut file = File::create(git_dir.join("config"))?;
    file.write_all(b"[config]\n")?;
    file.write_all(b"    bare = false\n")?;
    file.write_all(b"    repositoryformatversion = 0\n")?;
    file.write_all(b"    filemode = true\n")?;
    file.write_all(b"    logallrefupdates = true\n")?;

    let mut file = File::create(git_dir.join("description"))?;
    file.write_all(b"Unnamed repository; edit this file 'description' to name the repository.\n")?;

    fs::create_dir_all(git_dir.join("hooks"))?;
    fs::create_dir_all(git_dir.join("info"))?;
    fs::create_dir_all(git_dir.join("objects/info"))?;
    fs::create_dir_all(git_dir.join("objects/pack"))?;
    fs::create_dir_all(git_dir.join("refs"))?;

    Ok(())
}

#[derive(Error, Debug)]
enum HashObjectError {
    #[error("an io error occurred")]
    IoError(#[from] std::io::Error),
    #[error("not in a valid repo")]
    NoRepo,
}

fn hash_object(path: PathBuf) -> Result<(), HashObjectError> {
    let repo_dir = PathBuf::from("../gat-test");
    let git_dir = repo_dir.join(".git");

    println!("Hashing {path:#?}");

    let target = repo_dir.join(path);
    println!("    {target:#?}");

    Ok(())
}

fn main() -> Result<(), GatError> {
    let args = Cli::parse();

    match args.command {
        Commands::Init => init()?,
        Commands::HashObject { path } => hash_object(path)?,
    };
    Ok(())
}
