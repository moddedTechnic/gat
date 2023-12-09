use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InitialisationError {
    #[error("an io error occurred")]
    IoError(#[from] std::io::Error),
    #[error("a repository already exists")]
    ExistingRepo,
}

pub fn init() -> Result<(), InitialisationError> {
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
