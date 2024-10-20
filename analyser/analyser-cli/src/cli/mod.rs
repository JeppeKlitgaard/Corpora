use std::{env, fs::create_dir_all, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use eyre::Result;

mod analyse;
mod export;
mod fetch;
mod report;

fn get_default_working_directory() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("corpora");
    path
}

#[derive(Debug, Parser)]
#[command(name = "corporalyser")]
#[command(about = "Tool to fetch and analyse corpora in many languages")]
pub struct Cli {
    #[arg(
        global = true,
        short,
        long,
        value_name = "DIR",
        default_value = get_default_working_directory().into_os_string()
    )]
    working_directory: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Fetch(FetchArgs),

    #[command(arg_required_else_help = true)]
    Analyse(AnalyseArgs),

    #[command(arg_required_else_help = true)]
    Report { id: String },
    #[command(arg_required_else_help = true)]
    Export(ExportArgs),
}

#[derive(Debug, Args)]
struct FetchArgs {
    #[command(subcommand)]
    command: FetchCommands,

    #[arg(short, long, default_value_t = false, value_name = "?")]
    force: bool,
}

#[derive(Debug, Subcommand)]
enum FetchCommands {
    Wortschatz(FetchWortschatzArgs),
}

#[derive(Debug, Args)]
struct FetchWortschatzArgs {
    ids: Vec<String>,
}

#[derive(Debug, Args)]
struct AnalyseArgs {
    #[command(subcommand)]
    command: AnalyseCommands,

    #[arg(short, long, default_value_t = 3, value_name = "N")]
    ngram_n: usize,

    #[arg(short = 'k', long, default_value_t = 3, value_name = "K")]
    skipgram_n: usize,

    #[arg(short, long, default_value_t = true, value_name = "?")]
    show_progress: bool,

    #[arg(short, long, default_value_t = false, value_name = "?")]
    force: bool,
}

#[derive(Debug, Subcommand)]
enum AnalyseCommands {
    Wortschatz(AnalyseWortschatzArgs),
}

#[derive(Debug, Args)]
struct AnalyseWortschatzArgs {
    ids: Vec<String>,
}

#[derive(Debug, Args)]
struct ExportArgs {
    #[command(subcommand)]
    command: ExportCommands,

    #[arg(short, long, default_value_t = false, value_name = "?")]
    force: bool,
}

#[derive(Debug, Subcommand)]
enum ExportCommands {
    Oxeylyzer(ExportOxeylyzerArgs),
    Cmini(ExportCminiArgs),
}

#[derive(Debug, Args)]
struct ExportOxeylyzerArgs {
    id: String,
}

#[derive(Debug, Args)]
struct ExportCminiArgs {
    id: String,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();

    // Create working directory if missing
    let _ = create_dir_all(args.working_directory.as_path())?;
    let work_dir = args.working_directory.as_path();

    match args.command {
        Commands::Fetch(f_args) => match f_args.command {
            FetchCommands::Wortschatz(f_ws_args) => {
                for id in f_ws_args.ids {
                    fetch::wortschatz(&id, work_dir, f_args.force)?;
                }

                return Ok(());
            }
        },

        Commands::Analyse(a_args) => match a_args.command {
            AnalyseCommands::Wortschatz(a_ws_args) => {
                for id in a_ws_args.ids {
                    analyse::wortschatz(
                        &id,
                        work_dir,
                        a_args.ngram_n,
                        a_args.skipgram_n,
                        a_args.show_progress,
                        a_args.force,
                    )?;
                }

                return Ok(());
            }
        },
        Commands::Report { id } => report::report(&id, work_dir),
        Commands::Export(e_args) => match e_args.command {
            ExportCommands::Oxeylyzer(oxey_args) => {
                export::export_oxeylyzer(&oxey_args.id, work_dir, e_args.force)?;
                return Ok(());
            }
            ExportCommands::Cmini(cmini_args) => {
                export::export_cmini(&cmini_args.id, work_dir)?;
                return Ok(());
            }
        },
    }
}
