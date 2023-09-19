use eyre::Report;
use flate2::read::GzDecoder;
use std::io::Read;
use std::{fs::File, io, path::PathBuf};
use tar::Archive;

pub fn load_wortschatz_archive_sentences(archive_path: &PathBuf) -> Result<Vec<String>, Report> {
    let path_str = &archive_path.to_string_lossy();

    let archive_file = File::open(&archive_path)?;
    let decoded = GzDecoder::new(archive_file);
    let mut archive = Archive::new(decoded);

    let mut entries = archive.entries()?;
    let mut sentences_entry = entries
        .find_map(|x| {
            let entry = x.ok()?;
            let path = entry.path().ok()?;
            let filename = path.file_name()?;

            if filename.to_string_lossy().ends_with("-sentences.txt") {
                return Some(entry);
            }

            None
        })
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read sentences in archive {path_str}"),
        ))?;

    let mut sentences_str = String::new();
    sentences_entry.read_to_string(&mut sentences_str)?;

    let sentences = sentences_str
        .lines()
        .filter_map(|x| {
            let (_, content) = x.split_once('\t')?;
            Some(content.to_owned())
            // .ok_or(PyIOError::new_err(format!("Malformed sentence file in {path_str}")))
        })
        .collect();

    Ok(sentences)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::load_wortschatz_archive_sentences;
    use rayon::prelude::*;

    #[test]
    fn test_wortschatz_loader() {
        let path = PathBuf::from("sources/wortschatz/eng_news_2006_30K.tar.gz");
        load_wortschatz_archive_sentences(&path);
    }

    #[test]
    fn test_wortschatz_loader_parallel() {
        let paths = vec![
            PathBuf::from("sources/wortschatz/eng_news_2005_1M.tar.gz"),
            PathBuf::from("sources/wortschatz/eng_news_2006_1M.tar.gz"),
            PathBuf::from("sources/wortschatz/eng_news_2007_1M.tar.gz"),
            PathBuf::from("sources/wortschatz/eng_news_2008_1M.tar.gz"),
            PathBuf::from("sources/wortschatz/eng_news_2009_1M.tar.gz"),
        ];

        let sentences_raw: Vec<_> = paths
            .par_iter()
            .map(|path| {
                load_wortschatz_archive_sentences(path);
            })
            .collect();
    }

}