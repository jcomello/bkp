use anyhow::{Error, anyhow};
use clap::Parser;
use std::path::Path;
use std::fs;
use chrono::Utc;

#[derive(Parser)]
#[command(name = "bkp")]
#[command(version = "0.1.0")]
#[command(about = "Creates backup for single files", long_about = None)]
struct Cli {
    /// File path to be backed up
    #[arg(value_parser = validate_path)]
    path: String,

    /// Changes the date format for the backup file
    #[arg(long, short, default_value_t = "%Y%m%d%H%M".to_string())]
    format: String,
}

fn validate_path(s: &str) -> Result<String, Error> {
    let path = Path::new(s);

    match path.try_exists() {
        Ok(true) => Ok(s.to_string()),
        Ok(false) => Err(anyhow!("Path doesn't exists")),
        Err(e) => Err(e.into()),
    }
}

fn main() {
    let cli = Cli::parse();
    let source = &cli.path;
    let now = Utc::now().format(&cli.format);
    let destination = format!("{source}.{now}.bkp");

    let _ = fs::copy(cli.path, destination);
}
