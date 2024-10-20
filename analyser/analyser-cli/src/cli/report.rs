use std::fs::create_dir_all;
use std::io::{BufWriter, Write};
use std::{fs::File, io::BufReader, path::Path};

use chrono::Utc;
use eyre::Result;
use eyre::WrapErr;

use crate::objects::analysis::Analysis;
use crate::objects::report::{Report, ReportMetadata, ReportRecipe, ReportSourceType};
use crate::occurance::OccuranceAnalysis;
use crate::transforms::TransformSpecification;
use crate::utils::read_json;

impl Report {
    pub fn from_path(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let report: Report = serde_json::from_reader(reader)?;

        Ok(report)
    }

    pub fn from_id(id: &str, working_directory: &Path) -> Result<Self> {
        let mut report_path = working_directory.to_owned();
        report_path.push("report");
        report_path.push(format!("{id}.json"));

        Self::from_path(&report_path)
    }
}

pub fn read_recipe(path: &Path) -> Result<ReportRecipe> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let recipe: ReportRecipe = serde_json::from_reader(reader)?;

    Ok(recipe)
}

pub fn report(id: &str, working_directory: &Path) -> Result<()> {
    let mut recipe_path = working_directory.to_owned();
    recipe_path.push("recipe");
    recipe_path.push(format!("{id}.json"));

    let mut output_path = working_directory.to_owned();
    output_path.push("report");
    output_path.push(format!("{id}.json"));
    let _ = create_dir_all(&output_path.parent().unwrap());

    let recipe = read_recipe(&recipe_path)?;

    let total_weight: f64 = recipe.sources.iter().map(|x| x.weight).sum();
    let mut analysis_counts = OccuranceAnalysis::<usize>::default();
    let mut analysis_weighted_counts = OccuranceAnalysis::<f64>::default();

    for source in &recipe.sources {
        let id = &source.id;
        let trans_spec = TransformSpecification {
            strip_whitespace: source.strip_whitespace,
            strip_punctuation: source.strip_punctuation,
            strip_numbers: source.strip_numbers,
            strip_nonlatin: source.strip_nonlatin,
        };

        let analysis: OccuranceAnalysis<usize> = match source.type_ {
            ReportSourceType::Analysis => {
                let mut analysis_path = working_directory.to_owned();
                analysis_path.push("analysis");
                analysis_path.push(format!("{}.json", &id));

                let mut analysis: Analysis = read_json(&analysis_path)
                    .wrap_err_with(|| format!("Error reading analysis for ID '{}'. Maybe you didn't fetch and analyse this yet?", id))
                    ?;

                analysis.analysis.transform(&trans_spec);
                analysis.analysis.sort();
                analysis.analysis
            }
            ReportSourceType::Report => {
                let mut report_path = working_directory.to_owned();
                report_path.push("report");
                report_path.push(format!("{id}.json"));

                let report: Report = read_json(&report_path).wrap_err_with(|| format!("Error reading report for ID '{}'. Maybe you didn't generate the report for this this yet?", id))?;
                let mut analysis = report.analysis_counts;

                analysis.transform(&trans_spec);
                analysis.sort();
                analysis
            }
        };
        // Calculate ngram frequencies
        analysis_counts += analysis.clone();
        analysis_weighted_counts += analysis * (source.weight / total_weight);
    }

    // Establish frequencies
    let mut analysis_weighted_frequencies = analysis_weighted_counts.clone();
    analysis_weighted_frequencies.normalize();

    // Restore counts as a usize
    let mut analysis_weighted_counts: OccuranceAnalysis<usize> = analysis_weighted_counts.into();

    // Sort
    analysis_counts.sort();
    analysis_weighted_counts.sort();
    analysis_weighted_frequencies.sort();

    // Make report
    let metadata = ReportMetadata {
        id: recipe.metadata.id.clone(),
        name: recipe.metadata.name.clone(),
        languages: recipe.metadata.languages.clone(),
        version: recipe.metadata.version.clone(),
        extra: recipe.metadata.extra.clone(),
        process_date: Utc::now(),
    };
    let report = Report {
        metadata: metadata,
        sources: recipe.sources,
        count: 0,
        analysis_counts: analysis_weighted_counts,
        analysis_frequencies: analysis_weighted_frequencies,
    };

    let report_file = File::create(&output_path)?;

    let mut report_file_buf = BufWriter::new(report_file);

    serde_json::to_writer_pretty(&mut report_file_buf, &report)?;
    report_file_buf.flush()?;

    println!(
        "Finished making report. Report stored in {}",
        &output_path.display()
    );

    Ok(())
}
