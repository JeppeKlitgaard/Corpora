
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
enum AnalyserError {
    #[error("Could not read sentences in archive: {0}")]
    NoSentencesFound(#[from] io::Error),
}
