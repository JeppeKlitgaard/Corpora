use std::path::PathBuf;

use crate::io::load_wortschatz_archive_sentences;
use crate::objects::NGrams;
use counter::Counter;
use eyre::Report;
use indexmap::IndexMap;
use itertools::Itertools;
use rayon::prelude::*;
use sliding_windows::{IterExt, Storage};
use unicode_segmentation::UnicodeSegmentation;

// pub fn analyse(sentences: &Vec<String>, ngram_ns: Vec<u8>) -> NGrams {
//     let ngram_counters = ngram_ns.into_par_iter().map(|n| {
//         // let ngram: Vec<String> = sentences
//         //     .iter()
//         //     .map(|sentence| {
//         //         // let mut storage = Storage::new(n.into());
//         //         sentence
//         //             .graphemes(true)
//         //             // .sliding_windows(&mut storage)
//         //             // .map(|win| win.iter().join(""))
//         //             .collect::<Vec<_>>()
//         //     })
//         //     .flatten()
//         //     .map(|x| x.to_owned())
//         //     .collect();

//         let ngram: Counter<&str> = sentences
//             .into_iter()
//             .map(|sentence| sentence.graphemes(true).collect::<Vec<&str>>())
//             .flatten()
//             .collect::<Vec<&str>>()
//             .par_windows(n.into())
//             // .map(|x| x.to_owned())
//             .chunks(100)
//             .flatten()
//             .map(|x| Counter::from_iter(x.into_iter().map(|x| *x)))
//             .reduce(|| Counter::<&str>::new(), |acc, e| acc + e);
//         // .flatten();

//         // let ngram: Counter<String> = ngram.collect();
//         (n, ngram)
//     });

//     // Sort ngrams
//     let ngrams = ngram_counters
//         .map(|(n, counter)| {
//             let mut indexed_ngram: IndexMap<String, usize> = counter
//                 .into_iter()
//                 .map(|(gram, count)| (gram.to_owned(), count))
//                 .collect();

//             indexed_ngram.sort_by(|_k1, v1, _k2, v2| v1.cmp(v2).reverse());
//             (n, indexed_ngram)
//         })
//         .collect();

//     ngrams
// }

pub fn analyse(sentences: &Vec<String>, ngram_ns: Vec<u8>) -> NGrams {
    let ngram_counters = ngram_ns.into_par_iter().map(|n| {
        let ngram = sentences
            .iter()
            .map(|sentence| {
                let mut storage = Storage::new(n.into());
                sentence
                    .graphemes(true)
                    .sliding_windows(&mut storage)
                    .map(|win| win.iter().join(""))
                    .collect::<Vec<_>>()
            })
            .flatten();

        let ngram: Counter<String> = ngram.collect();
        (n, ngram)
    });

    let ngrams = ngram_counters
        .map(|(n, counter)| {
            let mut indexed_ngram: IndexMap<String, usize> = counter.into_iter().collect();
            indexed_ngram.sort_by(|_k1, v1, _k2, v2| v1.cmp(v2).reverse());
            (n, indexed_ngram)
        })
        .collect();

    ngrams
}

pub fn analyse_wortschatz(
    archive_paths: Vec<PathBuf>,
    ngram_ns: Vec<u8>,
) -> Result<NGrams, Report> {
    let nested_sentences: Result<Vec<Vec<String>>, Report> = archive_paths
        .par_iter()
        .map(|path| {
            let raw_sentences = load_wortschatz_archive_sentences(path)?;

            // Make sentences lower case
            let sentences: Vec<String> = raw_sentences
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect();

            Ok(sentences)
        })
        .collect();

    let sentences: Vec<String> = nested_sentences?.into_par_iter().flatten().collect();

    println!("Loaded sentences: {}", sentences.len());
    // Analyse ngrams
    Ok(analyse(&sentences, ngram_ns))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_analyse() {
        let sentences = vec!["Insp".to_owned(), "Brn".to_owned(), "Su".to_owned()];
        let ngram_ns = vec![1, 2, 3];

        let mut known_ngrams = HashMap::new();

        let mut known_ngrams_1 = IndexMap::new();
        known_ngrams_1.insert("n".to_owned(), 2);
        known_ngrams_1.insert("r".to_owned(), 1);
        known_ngrams_1.insert("s".to_owned(), 1);
        known_ngrams_1.insert("p".to_owned(), 1);
        known_ngrams_1.insert("I".to_owned(), 1);
        known_ngrams_1.insert("S".to_owned(), 1);
        known_ngrams_1.insert("u".to_owned(), 1);
        known_ngrams_1.insert("B".to_owned(), 1);

        let mut known_ngrams_2 = IndexMap::new();
        known_ngrams_2.insert("In".to_owned(), 1);
        known_ngrams_2.insert("Br".to_owned(), 1);
        known_ngrams_2.insert("rn".to_owned(), 1);
        known_ngrams_2.insert("sp".to_owned(), 1);
        known_ngrams_2.insert("ns".to_owned(), 1);
        known_ngrams_2.insert("Su".to_owned(), 1);

        let mut known_ngrams_3 = IndexMap::new();
        known_ngrams_3.insert("Brn".to_owned(), 1);
        known_ngrams_3.insert("nsp".to_owned(), 1);
        known_ngrams_3.insert("Su".to_owned(), 1);
        known_ngrams_3.insert("Ins".to_owned(), 1);

        known_ngrams.insert(1 as u8, known_ngrams_1);
        known_ngrams.insert(3 as u8, known_ngrams_3);
        known_ngrams.insert(2 as u8, known_ngrams_2);

        let ngrams = analyse(&sentences, ngram_ns);

        assert_eq!(ngrams, known_ngrams);
    }
}
