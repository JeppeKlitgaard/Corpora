use indexmap::IndexMap;
use serde::Serialize;

use crate::occurance::Countable;

use super::report::Report;

#[derive(Serialize, Debug)]
pub struct OxeylyserLanguageData {
    pub language: String,

    pub characters: IndexMap<Countable, f64>,
    pub bigrams: IndexMap<Countable, f64>,
    pub trigrams: IndexMap<Countable, f64>,

    pub skipgrams: IndexMap<Countable, f64>,
    pub skipgrams2: IndexMap<Countable, f64>,
    pub skipgrams3: IndexMap<Countable, f64>,
}

impl OxeylyserLanguageData {
    pub fn from_report(report: &Report) -> Self {
        Self {
            language: report.metadata.id.clone(),

            characters: report.analysis_frequencies.ngrams[&1].clone().into(),
            bigrams: report.analysis_frequencies.ngrams[&2].clone().into(),
            trigrams: report.analysis_frequencies.ngrams[&3].clone().into(),

            skipgrams: report.analysis_frequencies.skipgrams[&1].clone().into(),
            skipgrams2: report.analysis_frequencies.skipgrams[&2].clone().into(),
            skipgrams3: report.analysis_frequencies.skipgrams[&3].clone().into(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CminiLanguageData {
    pub monograms: IndexMap<Countable, usize>,
    pub bigrams: IndexMap<Countable, usize>,
    pub trigrams: IndexMap<Countable, usize>,
    pub words: IndexMap<Countable, usize>,
}

impl CminiLanguageData {
    pub fn from_report(report: &Report) -> Self {
        Self {
            monograms: report.analysis_counts.ngrams[&1].clone().into(),
            bigrams: report.analysis_counts.ngrams[&2].clone().into(),
            trigrams: report.analysis_counts.ngrams[&3].clone().into(),
            words: report.analysis_counts.words.clone().into(),
        }
    }
}
