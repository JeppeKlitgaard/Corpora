use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::occurance::OccuranceAnalysis;

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub metadata: ReportMetadata,
    pub sources: Vec<ReportSource>,

    pub count: u64,
    pub analysis_counts: OccuranceAnalysis<usize>,
    pub analysis_frequencies: OccuranceAnalysis<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportMetadata {
    pub id: String,
    pub name: String,
    pub languages: Vec<String>,
    pub version: Version,
    pub extra: Value,
    pub process_date: DateTime<Utc>,
}

// Recipe
#[derive(Serialize, Deserialize, Debug)]
pub struct ReportRecipeMetadata {
    pub id: String,
    pub name: String,
    pub languages: Vec<String>,
    pub version: Version,
    pub extra: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportRecipe {
    pub metadata: ReportRecipeMetadata,
    pub sources: Vec<ReportSource>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportSource {
    pub id: String,
    pub weight: f64,

    #[serde(rename = "type")]
    pub type_: ReportSourceType,

    #[serde(default)]
    pub strip_whitespace: bool,

    #[serde(default)]
    pub strip_punctuation: bool,

    #[serde(default)]
    pub strip_numbers: bool,

    #[serde(default)]
    pub strip_nonlatin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReportSourceType {
    Report,
    Analysis,
}
