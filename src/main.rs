mod clip;
mod utils;
mod watermark;

use clap::{Parser, Subcommand};
use clip::{cut_clip, run_job};
use owo_colors::OwoColorize;
use std::{io::Write, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a job file. Optionally pass a filename for the YAML job file.
    run {
        /// The YAML file. Defaults to exhibits.yaml if no filename is given.
        // #[arg(long)]
        file: Option<String>,
    },

    /// Create a new job file. Optionally pass a filename for the YAML file.
    new {
        /// The YAML file. Defaults to exhibits.yaml if no filename is given.
        // #[arg(long)]
        file: Option<String>,
    },
}
fn main() {
    let cli = Cli::parse();
    let default_job_file_name = "exhibits.yaml";

    match &cli.command {
        Commands::run { file } => {
            let mut job_file = default_job_file_name;

            if let Some(f) = file {
                job_file = f;
            }
            println!("Running job file: {}", job_file);

            run_job(job_file);
        }
        Commands::new { file } => {
            let mut job_file = default_job_file_name;

            if let Some(f) = file {
                job_file = f;
            }

            let mut file = std::fs::File::create(job_file).unwrap();

            let yaml_bytes = include_bytes!("exhibits.yaml");

            let _ = file.write_all(yaml_bytes);
            println!("New job file created: {}", job_file.bright_green());
        }
    }
}
