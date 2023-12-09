use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

use libflate::zlib;
use sha1::{Digest, Sha1};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashObjectError {
    #[error("an io error occurred")]
    IoError(#[from] std::io::Error),
    #[error("not in a valid repo")]
    NoRepo,
}

fn read_target(target: PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(target)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn hash_blob(content: impl AsRef<[u8]>) -> Vec<u8> {
    let content = content.as_ref();
    let header = format!("blob {}", content.len());
    let mut hasher = Sha1::new();
    hasher.update(header);
    hasher.update(b"\0");
    hasher.update(content);
    hasher.finalize().into_iter().collect()
}

fn encode_blob(content: impl AsRef<[u8]>) -> Result<Vec<u8>, HashObjectError> {
    let content = content.as_ref();
    let mut encoder = zlib::Encoder::new(Vec::new())?;
    encoder.write_all(format!("blob {}", content.len()).as_bytes())?;
    encoder.write_all(&[0])?;
    encoder.write_all(content)?;
    Ok(encoder.finish().into_result()?)
}

pub fn hash_object(path: PathBuf) -> Result<(), HashObjectError> {
    let repo_dir = PathBuf::from("../gat-test");
    let git_dir = repo_dir.join(".git");

    if !std::path::Path::new(&git_dir).exists() {
        return Err(HashObjectError::NoRepo);
    }

    println!("Hashing {path:#?}");

    let target = repo_dir.join(path);
    let buf = read_target(target)?;

    let mut hash_parts = hash_blob(&buf).into_iter().map(|x| format!("{x:02x}"));
    let hash_dir: String = hash_parts.next().expect("20 bytes in the hash");
    let hash_file: String = hash_parts.collect();

    let hash_dir = git_dir.join("objects").join(hash_dir);
    fs::create_dir_all(&hash_dir)?;

    let mut file = File::create(hash_dir.join(hash_file))?;
    file.write_all(encode_blob(buf)?.as_slice())?;

    Ok(())
}
