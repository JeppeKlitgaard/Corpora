use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::occurance::OccuranceAnalysis;

#[derive(Serialize, Deserialize, Debug)]
pub struct Analysis {
    pub source: AnalysisSource,
    pub metadata: AnalysisMetadata,
    pub analysis: OccuranceAnalysis<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisSource {
    pub origin_id: String,
    pub origin_name: String,
    pub origin_url: Url,
    pub license: String,

    pub date: DateTime<Utc>,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisMetadata {
    pub date: DateTime<Utc>,
}
