use std::{fs::File, io::BufReader, path::Path};

use crate::crypt::digest_sha256;
use eyre::Result;
use ring::digest::Digest;
use chrono::{DateTime, Utc};
use std::fs;

pub fn file_sha256(path: &Path) -> Result<Digest> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = digest_sha256(reader)?;

    Ok(digest)
}


pub fn get_mtime(path: &Path) -> Result<DateTime<Utc>> {
    let as_epoch = fs::metadata(path)?.modified()?;
    Ok(as_epoch.into())
}