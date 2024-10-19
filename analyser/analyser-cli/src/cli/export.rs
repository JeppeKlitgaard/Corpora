use eyre::Result;
use std::{
    fs::{create_dir, File}, io::Write, path::{Path, PathBuf}
};

use crate::objects::{export::{CminiLanguageData, OxeylyserLanguageData}, report::Report};

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

pub fn export_cmini(report: &Report, working_directory: &Path, force: bool) -> Result<()> {
    let cmini_output = CminiLanguageData::from_report(report);

    let mut export_path: PathBuf = working_directory.to_owned();
    export_path.push("export_cmini");
    if !export_path.exists() {
        create_dir(&export_path)?;
    }

    let monograms_path = export_path.join("monograms.json");
    let monograms_str = serde_json::to_string_pretty(&cmini_output.monograms)?;
    let mut monograms_file = File::create(&monograms_path)?;
    monograms_file.write_all(monograms_str.as_bytes())?;
    monograms_file.flush()?;

    let bigrams_path = export_path.join("bigrams.json");
    let bigrams_str = serde_json::to_string_pretty(&cmini_output.bigrams)?;
    let mut bigrams_file = File::create(&bigrams_path)?;
    bigrams_file.write_all(bigrams_str.as_bytes())?;
    bigrams_file.flush()?;

    let trigrams_path = export_path.join("trigrams.json");
    let trigrams_str = serde_json::to_string_pretty(&cmini_output.trigrams)?;
    let mut trigrams_file = File::create(&trigrams_path)?;
    trigrams_file.write_all(trigrams_str.as_bytes())?;
    trigrams_file.flush()?;

    let words_path = export_path.join("words.json");
    let words_str = serde_json::to_string_pretty(&cmini_output.words)?;
    let mut words_file = File::create(&words_path)?;
    words_file.write_all(words_str.as_bytes())?;
    words_file.flush()?;

    Ok(())
}

//     let mut export_path: PathBuf = working_directory.to_owned();
//     // export_path.push(id);
//     export_path.push("export_cmini.json");

//     if !force && export_path.exists() {
//         // println!("Sentences for ID '{id}' already exists. To redownload, use 'force' argument");
//         return Ok(());
//     }

//     let export_file = File::create(&export_path)?;
//     serde_json::to_writer_pretty(&export_file, &cmini_output)?;

//     // // println!("Fetching Wortschatz Corpora with ID: {id}");

//     // let url = Url::parse(&format!(
//     //     "https://downloads.wortschatz-leipzig.de/corpora/{id}.tar.gz"
//     // ))
//     // .expect("invalid url.");

//     // _ = create_dir_all(sentences_path.parent().unwrap());

//     // download_and_decompress_archive(url, sentences_path.as_path())?;

//     // println!(
//     //     "Fetched Wortschatz Corpus '{}' and stored at '{}",
//     //     id,
//     //     &sentences_path.display(),
//     // );

//     Ok(())
// }
