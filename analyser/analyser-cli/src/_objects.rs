
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::occurance::{Countable, OccuranceAnalysis};

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