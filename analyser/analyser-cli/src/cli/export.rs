use eyre::Result;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::objects::{export::OxeylyserLanguageData, report::Report};

pub fn export_oxeylyzer(id: &str, working_directory: &Path, force: bool) -> Result<()> {
    let report = Report::from_id(id, working_directory)?;
    let oxey_output = OxeylyserLanguageData::from_report(&report);

    let mut export_path: PathBuf = working_directory.to_owned();
    export_path.push("export");
    export_path.push("oxeylyzer");
    export_path.push(format!("{}.json", id));

    if !force && export_path.exists() {
        return Ok(());
    }

    let export_file = File::create(&export_path)?;
    serde_json::to_writer_pretty(&export_file, &oxey_output)?;

    Ok(())
}

pub fn export_cmini(id: &str, working_directory: &Path) -> Result<()> {
    let report = Report::from_id(id, working_directory)?;

    let monograms = &report.analysis_counts.ngrams[&1];
    let bigrams = &report.analysis_counts.ngrams[&2];
    let trigrams = &report.analysis_counts.ngrams[&3];
    let words = &report.analysis_counts.words;

    let mut export_path: PathBuf = working_directory.to_owned();
    export_path.push("export");
    export_path.push("cmini");
    export_path.push(id);
    if !export_path.exists() {
        create_dir_all(&export_path)?;
    }

    let monograms_path = export_path.join("monograms.json");
    let monograms_str = serde_json::to_string_pretty(monograms)?;
    let mut monograms_file = File::create(&monograms_path)?;
    monograms_file.write_all(monograms_str.as_bytes())?;
    monograms_file.flush()?;

    let bigrams_path = export_path.join("bigrams.json");
    let bigrams_str = serde_json::to_string_pretty(bigrams)?;
    let mut bigrams_file = File::create(&bigrams_path)?;
    bigrams_file.write_all(bigrams_str.as_bytes())?;
    bigrams_file.flush()?;

    let trigrams_path = export_path.join("trigrams.json");
    let trigrams_str = serde_json::to_string_pretty(trigrams)?;
    let mut trigrams_file = File::create(&trigrams_path)?;
    trigrams_file.write_all(trigrams_str.as_bytes())?;
    trigrams_file.flush()?;

    let words_path = export_path.join("words.json");
    let words_str = serde_json::to_string_pretty(words)?;
    let mut words_file = File::create(&words_path)?;
    words_file.write_all(words_str.as_bytes())?;
    words_file.flush()?;

    Ok(())
}
