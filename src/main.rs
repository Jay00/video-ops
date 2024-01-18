mod clip;
mod utils;
mod watermark;

use clap::{Parser, Subcommand};
use clip::{ffmpeg_is_on_path, run_job};
use owo_colors::OwoColorize;
use std::io::Write;

#[derive(Parser)]
#[command(name = "clipper")]
#[command(bin_name = "clipper")]
#[command(author = "Jason K. Clark <jasonclarklaw.com>")]
#[command(version)]
#[command(about = "\n\n** CLIPPER **\nAn easy to use clip maker.\nQuickly create exhibit clips with labels from a simple yaml text file.\nCreated by Jason K. Clark", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a job file to create clips.
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
            if !ffmpeg_is_on_path() {
                eprintln!(
                    "{}",
                    "FFmpeg not found! FFmpeg must be installed and available on PATH."
                        .bright_red()
                );

                eprintln!(
                    "{}",
                    "You must install FFmpeg: https://ffmpeg.org/download.html".bright_yellow()
                );
                std::process::exit(1);
            }
            let mut job_file = default_job_file_name;

            if let Some(f) = file {
                job_file = f;
            }
            println!("Running job file: {}", job_file.bright_cyan());

            run_job(job_file);
            println!("Completed job file: {}", job_file.bright_cyan());
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
