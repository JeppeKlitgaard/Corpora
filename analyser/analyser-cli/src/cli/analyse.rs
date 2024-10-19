use crate::{
    crypt::DigestExt,
    io::get_mtime,
    objects::analysis::{Analysis, AnalysisMetadata, AnalysisSource},
    utils::read_json,
};
use chrono::{self};
use data_encoding::HEXUPPER;
use eyre::{eyre, Result};
use humantime::format_duration;
use rayon::prelude::*;
use ring::digest::Digest;
use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::Instant,
};
use url::Url;

impl DigestExt for Digest {
    fn to_str(&self) -> String {
        HEXUPPER.encode(self.as_ref())
    }
}

use crate::{analyse::analyse, io::file_sha256};

pub fn wortschatz(
    id: &str,
    working_directory: &Path,
    ngram_n: usize,
    skipgram_n: usize,
    show_progress: bool,
    force: bool,
) -> Result<()> {
    let start_time = Instant::now();
    println!("Analysing corpus: '{id}'...");

    // Fetch sentences
    let mut corpus_path: PathBuf = working_directory.to_owned();
    corpus_path.push("data");
    corpus_path.push(id);

    let mut sentences_path = corpus_path.clone();
    sentences_path.push("sentences.txt");

    if !sentences_path.exists() {
        return Err(eyre!(
            "Could not open '{}'. Maybe you need to fetch it first?",
            &sentences_path.display()
        ));
    }

    let mut analysis_path = corpus_path.clone();
    analysis_path.push("analysis.json");

    let existing_sha256: Result<String> =
        (|| Ok(read_json::<Analysis>(&analysis_path)?.source.hash))();

    // Compute SHA256 of sentence file
    let sha256 = file_sha256(&sentences_path)?;

    if !force && Some(sha256.to_str()) == existing_sha256.ok() {
        println!("Corpus was already analysed.");
        return Ok(());
    }

    // Load sentences
    let raw_sentences = read_to_string(&sentences_path)?;
    let sentences: Vec<String> = raw_sentences
        .par_lines()
        .filter_map(|s| {
            let (_, content) = s.split_once('\t')?;
            Some(content.to_owned().to_lowercase())
        })
        .collect();

    let ngram_ns: Vec<usize> = (1..=ngram_n).collect();
    let skipgram_ns: Vec<usize> = (1..=skipgram_n).collect();
    let occurance_analysis = analyse(&sentences, ngram_ns, skipgram_ns, show_progress);

    // Construct analysis
    let analysis = Analysis {
        source: AnalysisSource {
            hash: sha256.to_str(),
            license: "CC BY-NC".to_owned(),
            origin_id: "wortschatz".to_owned(),
            origin_name: "Deutsche Wortschatz by Institut fűr Informatik at Universität Leipzig"
                .to_owned(),
            origin_url: Url::parse("https://wortschatz.uni-leipzig.de/en")?,
            date: get_mtime(&sentences_path)?,
        },
        metadata: AnalysisMetadata {
            date: chrono::Utc::now(),
        },
        analysis: occurance_analysis,
    };

    let analysis_file = File::create(&analysis_path)?;
    let mut analysis_file_buf = BufWriter::new(analysis_file);

    serde_json::to_writer_pretty(&mut analysis_file_buf, &analysis)?;
    analysis_file_buf.flush()?;

    let analysis_stats_strs_sentences_words = vec![
        format!("{} sentences", analysis.analysis.num_sentences),
        format!("{} words", analysis.analysis.words.sum()),
    ];

    let analysis_stats_strs_ngrams: Vec<_> = analysis
        .analysis
        .ngrams
        .iter()
        .map(|(n, x)| format!("{} {}-grams", x.sum(), n))
        .collect();

    let analysis_stats_strs_skipgrams: Vec<_> = analysis
        .analysis
        .skipgrams
        .iter()
        .map(|(n, x)| format!("{} {}-skipgrams", x.sum(), n))
        .collect();

    let mut analysis_stats_strs = Vec::<_>::new();
    analysis_stats_strs.extend(analysis_stats_strs_sentences_words);
    analysis_stats_strs.extend(analysis_stats_strs_ngrams);
    analysis_stats_strs.extend(analysis_stats_strs_skipgrams);

    let mut analysis_stat_str = analysis_stats_strs.join(", ");

    let analysis_rate = analysis.analysis.num_sentences as f64 / start_time.elapsed().as_secs_f64();

    let elapsed_time = start_time.elapsed();
    analysis_stat_str.push_str(&format!(
        " analysed in {} ({:.2} sentences/sec)",
        format_duration(elapsed_time),
        analysis_rate
    ));

    println!("Finished analysing corpus: {}", analysis_stat_str);
    println!("Analysis stored in {}", analysis_path.display());

    Ok(())
}
