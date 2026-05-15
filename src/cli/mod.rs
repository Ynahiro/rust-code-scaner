use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "codescanner")]
#[command(about = "Приложение для сканнирования и анализа кода", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Dashboard(DashboardArgs),
    Scan(ScanArgs),
    Report(ReportArgs),
    Source(SourceArgs),
    User(UserArgs),
    Config(ConfigArgs),
}

#[derive(Parser)]
pub struct DashboardArgs {
    #[arg(long, default_value_t = false)]
    pub show: bool,
}

#[derive(Parser)]
pub struct ScanArgs {
    #[arg(long, default_value_t = false)]
    pub interactive: bool,

    #[arg(long)]
    pub source: Option<SourceType>,

    #[arg(long)]
    pub url: Option<String>,

    #[arg(long, value_delimiter = ',')]
    pub lang: Vec<Lang>,

    #[arg(long)]
    pub min_similarity: Option<f64>,

    #[arg(long, value_delimiter = ',')]
    pub licenses: Vec<String>,
}

#[derive(ValueEnum, Clone)]
pub enum SourceType {
    Git,
    Archive,
    Local,
}

#[derive(ValueEnum, Clone)]
pub enum Lang {
    Rust,
    Python,
    Java,
    C,
}

#[derive(Subcommand)]
pub enum ReportArgs {
    List,
    Show {
        id: i64,
    },
    Export {
        id: i64,

        #[arg(long, value_enum)]
        format: ExpotFormat,

        #[arg(long)]
        output: PathBuf,
    },
    Diff {
        id1: i64,
        id2: i64,
    },
}

#[derive(ValueEnum, Clone)]
pub enum ExportFormat {
    Pdf,
    Html,
    Json,
}

