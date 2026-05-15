use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "codescanner")]
#[command(about = "Приложение для сканнирования и анализа кода", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Dashboard(DashboardArgs),
    Scan(ScanArgs),
    #[command(subcommand)]
    Report(ReportArgs),
    #[command(subcommand)]
    Source(SourceArgs),
    #[command(subcommand)]
    User(UserArgs),
    #[command(subcommand)]
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
        format: ExportFormat,

        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(ValueEnum, Clone)]
pub enum ExportFormat {
    Pdf,
    Html,
    Json,
}

#[derive(Subcommand)]
pub enum SourceArgs {
    List,
    Add {
        #[arg(long)]
        name: String,
        url_or_path: String,
    },
    Update {
        id: i64,
    },
    Remove {
        id: i64,
    },
    Status,
}

#[derive(Subcommand)]
pub enum UserArgs {
    List,
    Create {
        name: String,
        #[arg(long, value_enum)]
        role: UserRole,
    },
    Delete {
        name: String,
    },
    SetRole {
        name: String,
        #[arg(long, value_enum)]
        role: UserRole,
    },
    ResetPassword {
        name: String,
    },
}

#[derive(ValueEnum, Clone)]
pub enum UserRole {
    Admin,
    Analyst,
    Operator,
}

#[derive(Subcommand)]
pub enum ConfigArgs {
    Show,
    Set { key: String, value: String },
    Reset,
}
