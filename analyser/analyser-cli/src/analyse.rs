use linya::{Bar, Progress};
use rayon::prelude::*;
use std::sync::Mutex;
use unicode_segmentation::UnicodeSegmentation;

use crate::occurance::{
    Countable, OccuranceAnalysis, OccuranceCounter, Occurances,
};

pub fn analyse(
    sentences: &Vec<String>,
    ngram_ns: Vec<usize>,
    skipgram_ns: Vec<usize>,
    show_progress: bool,
) -> OccuranceAnalysis<usize> {
    let progress: Option<Mutex<Progress>>;
    let bar: Option<Bar>;

    if show_progress {
        progress = Some(Mutex::new(Progress::new()));
        bar = Some(
            progress
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .bar(sentences.len(), format!("Processing: ")),
        );
    } else {
        progress = None;
        bar = None;
    }

    let skipgram_ns_incremented: Vec<usize> = skipgram_ns.iter().map(|&x| x + 2).collect();
    let ngram_ns: Vec<usize> = ngram_ns.iter().map(|&x| x as usize).collect();

    let mut window_size: Vec<usize> = [skipgram_ns_incremented, ngram_ns.clone()].concat();
    window_size.sort_unstable(); // Sort the vector to prepare for deduplication
    window_size.dedup(); // Remove duplicates

    let mut occ_analysis: OccuranceAnalysis<usize> = sentences
        .par_iter()
        .fold(
            || OccuranceAnalysis::default(),
            |mut occ_analysis: OccuranceAnalysis<usize>, sentence: &String| {
                let graphemes = sentence.graphemes(true).collect::<Vec<_>>();

                ngram_ns.iter().for_each(|n| {
                    let windows = graphemes.windows(*n);

                    let grams: OccuranceCounter = windows
                        .into_iter()
                        .filter(|&x| x.len() == *n)
                        // Iterator over arrays of graphemes
                        .map(|x| Countable::from(x.join("")))
                        .collect::<OccuranceCounter>();

                    let ngrams_map: Occurances<_> = grams.into();
                    let ngrams_entry = occ_analysis.ngrams.entry(*n).or_default();
                    *ngrams_entry += ngrams_map;
                });

                skipgram_ns.iter().for_each(|n| {
                    let skip_n = n + 2;
                    let windows = graphemes.windows(skip_n);

                    let skipgrams: OccuranceCounter = windows
                        .into_iter()
                        .filter(|&x| x.len() == skip_n)
                        // Iterator over arrays of graphemes
                        .map(|x| {
                            let first_last = [*x.first().unwrap(), *x.last().unwrap()];
                            Countable::from(first_last.join(""))
                        })
                        .collect::<OccuranceCounter>();

                    let skipgrams_map: Occurances<_> = skipgrams.into();
                    let skipgrams_entry = occ_analysis.skipgrams.entry(*n).or_default();
                    *skipgrams_entry += skipgrams_map;
                });

                if show_progress {
                    progress
                        .as_ref()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .inc_and_draw(bar.as_ref().unwrap(), 1);
                }

                occ_analysis
            },
        )
        .reduce(
            || OccuranceAnalysis::default(),
            |mut occ_analysis1: OccuranceAnalysis<usize>,
             occ_analysis2: OccuranceAnalysis<usize>| {
                for (n, entry2) in occ_analysis2.ngrams {
                    let entry1 = occ_analysis1.ngrams.entry(n).or_default();

                    *entry1 += entry2.clone();
                }

                for (n, entry2) in occ_analysis2.skipgrams {
                    let entry1 = occ_analysis1.skipgrams.entry(n).or_default();

                    *entry1 += entry2.clone();
                }
                occ_analysis1
            },
        );

    occ_analysis.sort();

    dbg!(&occ_analysis);

    // ngrams
    occ_analysis

    // ngrams
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use super::*;

//     #[test]
//     fn test_analyse() {
//         let sentences = vec!["Insp".to_owned(), "Brn".to_owned(), "Su".to_owned()];
//         let ngram_ns = vec![1, 2, 3];

//         let mut known_ngrams = NGramLike::<usize>::new();

//         let mut known_ngrams_1 = IndexMap::new();
//         known_ngrams_1.insert("n".to_owned(), 2);
//         known_ngrams_1.insert("r".to_owned(), 1);
//         known_ngrams_1.insert("s".to_owned(), 1);
//         known_ngrams_1.insert("p".to_owned(), 1);
//         known_ngrams_1.insert("I".to_owned(), 1);
//         known_ngrams_1.insert("S".to_owned(), 1);
//         known_ngrams_1.insert("u".to_owned(), 1);
//         known_ngrams_1.insert("B".to_owned(), 1);

//         let mut known_ngrams_2 = IndexMap::new();
//         known_ngrams_2.insert("In".to_owned(), 1);
//         known_ngrams_2.insert("Br".to_owned(), 1);
//         known_ngrams_2.insert("rn".to_owned(), 1);
//         known_ngrams_2.insert("sp".to_owned(), 1);
//         known_ngrams_2.insert("ns".to_owned(), 1);
//         known_ngrams_2.insert("Su".to_owned(), 1);

//         let mut known_ngrams_3 = IndexMap::new();
//         known_ngrams_3.insert("Brn".to_owned(), 1);
//         known_ngrams_3.insert("nsp".to_owned(), 1);
//         known_ngrams_3.insert("Ins".to_owned(), 1);

//         known_ngrams.insert(1 as u8, known_ngrams_1);
//         known_ngrams.insert(2 as u8, known_ngrams_2);
//         known_ngrams.insert(3 as u8, known_ngrams_3);

//         let ngrams = analyse(&sentences, ngram_ns, false);

//         assert_eq!(ngrams, known_ngrams);
//     }
// }
