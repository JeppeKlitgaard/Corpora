use std::{fs::File, io::BufReader, path::Path};

use eyre::Result;
use serde::de::DeserializeOwned;

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let obj: T = serde_json::from_reader(reader)?;

    Ok(obj)
}
