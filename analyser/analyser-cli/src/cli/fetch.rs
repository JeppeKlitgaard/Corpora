use eyre::Result;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use url::Url;

use crate::http::download_and_decompress_archive;

pub fn wortschatz(id: &str, working_directory: &Path, force: bool) -> Result<()> {
    let mut sentences_path: PathBuf = working_directory.to_owned();
    sentences_path.push("fetch");
    sentences_path.push("wortschatz");
    sentences_path.push(id);
    sentences_path.push("sentences.txt");

    if !force && sentences_path.exists() {
        println!("Sentences for ID '{id}' already exists. To redownload, use 'force' argument");
        return Ok(());
    }

    println!("Fetching Wortschatz Corpora with ID: {id}");

    let url = Url::parse(&format!(
        "https://downloads.wortschatz-leipzig.de/corpora/{id}.tar.gz"
    ))
    .expect("invalid url.");

    _ = create_dir_all(sentences_path.parent().unwrap());

    download_and_decompress_archive(url, sentences_path.as_path())?;

    println!(
        "Fetched Wortschatz Corpus '{}' and stored at '{}",
        id,
        &sentences_path.display(),
    );

    Ok(())
}
