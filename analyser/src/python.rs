use std::path::PathBuf;

use pyo3::types::PyModule;
use pyo3::{pyfunction, PyResult, wrap_pyfunction, pymodule, Python};
use crate::objects::NGrams;
use eyre::Report;
use crate::analyse as analyse_mod;
use crate::io::load_wortschatz_archive_sentences;
use rayon::prelude::*;


#[pyfunction]
fn analyse(sentences: Vec<String>, ngram_ns: Vec<u8>) -> PyResult<NGrams> {
    let ngrams = analyse_mod::analyse(&sentences, ngram_ns);

    Ok(ngrams)
}

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn analyse_wortschatz(archive_paths: Vec<PathBuf>, ngram_ns: Vec<u8>) -> PyResult<NGrams> {
//     let nested_sentences: Result<Vec<Vec<String>>, Report> = archive_paths
//         .par_iter()
//         .map(|path| {
//             let raw_sentences = load_wortschatz_archive_sentences(path)?;

//             // Make sentences lower case
//             let sentences: Vec<String> = raw_sentences
//                 .into_par_iter()
//                 .map(|s| s.to_lowercase())
//                 .collect();

//             Ok(sentences)
//         })
//         .collect();

//     let sentences: Vec<String> = nested_sentences?.into_par_iter().flatten().collect();

//     // Analyse ngrams
//     Ok(analyse_mod::analyse(&sentences, ngram_ns))
// }

#[pyfunction]
fn analyse_wortschatz(archive_paths: Vec<PathBuf>, ngram_ns: Vec<u8>) -> PyResult<NGrams> {
    Ok(analyse_mod::analyse_wortschatz(archive_paths, ngram_ns)?)
}

/// A Python module implemented in Rust.
#[pymodule]
fn analyser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyse, m)?)?;
    m.add_function(wrap_pyfunction!(analyse_wortschatz, m)?)?;
    Ok(())
}