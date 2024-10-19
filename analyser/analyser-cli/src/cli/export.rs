use eyre::Result;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::objects::{export::OxeylyserLanguageData, report::Report};

pub fn export_oxeylyzer(report: &Report, working_directory: &Path, force: bool) -> Result<()> {
    let oxey_output = OxeylyserLanguageData::from_report(report);

    let mut export_path: PathBuf = working_directory.to_owned();
    // export_path.push(id);
    export_path.push("export_oxeylyzer.json");

    if !force && export_path.exists() {
        // println!("Sentences for ID '{id}' already exists. To redownload, use 'force' argument");
        return Ok(());
    }

    let export_file = File::create(&export_path)?;
    serde_json::to_writer_pretty(&export_file, &oxey_output)?;

    // // println!("Fetching Wortschatz Corpora with ID: {id}");

    // let url = Url::parse(&format!(
    //     "https://downloads.wortschatz-leipzig.de/corpora/{id}.tar.gz"
    // ))
    // .expect("invalid url.");

    // _ = create_dir_all(sentences_path.parent().unwrap());

    // download_and_decompress_archive(url, sentences_path.as_path())?;

    // println!(
    //     "Fetched Wortschatz Corpus '{}' and stored at '{}",
    //     id,
    //     &sentences_path.display(),
    // );

    Ok(())
}
