mod clip;
mod utils;
mod watermark;

use clap::{Parser, Subcommand};
use clip::{cut_clip, run_job};
use std::{io::Write, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a job file
    run {
        /// The document id
        #[arg(long)]
        file: Option<String>,
    },

    /// Make new job file
    new {},
}
fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();

    match &cli.command {
        Commands::run { file } => {
            println!("Running job_file: {:?}", file);
        }
        Commands::new {} => {
            println!("Making new file.");

            let mut file = std::fs::File::create("exhibits.yaml").unwrap();

            let yaml = r#"#Comment: Clips
#Describe Clips that you want to cut

# Time Stamps: Note that you can use two different time unit formats: sexagesimal (HOURS:MM:SS.MILLISECONDS, as in 01:23:45.678),
# or in seconds. If a fraction is used, such as 02:30.05, this is interpreted as "5 100ths of a second", not as frame 5.
# For instance, 02:30.5 would be 2 minutes, 30 seconds, and a half a second, which would be the same as using 150.5 in seconds.
---
output_directory: "./test/exhibits"
case_number: "2020 CF1 123123"
clips:
  - label: "Ex. 12A"
    source: "./test/a.mp4"
    start: "00:00:00.000" #first list item
    stop: "00:00:05.100"

  - label: "Ex. 12C"
    source: "./test/a.mp4"
    start: "00:00:00.000" #first list item
    stop: "00:00:05.100"

  - label: "Ex. 14A"
    source: "./test/a.mp4"
    start: "00:00:00.000" #first list item
    stop: "00:00:05.100"
    remove_audio: true
"#;
            let _ = file.write_all(yaml.as_bytes());
        }
    }
}
