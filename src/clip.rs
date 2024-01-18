use crate::utils::COURIER_BOLD;
use clap::builder::Str;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use spinoff::{spinners, Color, Spinner};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum LabelPosition {
    BottomLeft,
    BottomMiddle,
    BottomRight,
    TopLeft,
    TopMiddle,
    TopRight,
}

struct Job {
    output_directory: PathBuf,
    clips: Vec<Clip>,
}

#[derive(Debug, PartialEq)]
struct Clip {
    source: PathBuf,
    label: String,
    start: String,
    stop: String,
    remove_audio: bool,
    label_position: LabelPosition,
    label_display: bool,
}

pub fn run_job(yaml_file: &str) {
    let job = read_yaml_file(yaml_file);

    for clip in job.clips {
        let filename = format!(
            "{}.{}",
            &clip.label,
            &clip.source.extension().unwrap().to_str().unwrap()
        );
        let destination = job.output_directory.join(filename);

        println!("Label: {}", clip.label);
        println!("Destination Path: {:?}", destination.bright_purple());

        let _ = cut_clip(
            &clip.source,
            &destination,
            &clip.label,
            &clip.start,
            &clip.stop,
            clip.remove_audio,
            clip.label_position,
            clip.label_display,
        );
    }
}

fn make_clip(map: &Mapping) -> Clip {
    let mut source: PathBuf = PathBuf::new();
    let mut label: String = String::new();
    let mut start: String = String::new();
    let mut stop: String = String::new();
    let mut remove_audio: bool = false;
    let mut label_position: LabelPosition = LabelPosition::BottomMiddle;
    let mut label_display: bool = true;

    // println!("{:?}", map);
    for (k, v) in map {
        let k = k.as_str().unwrap();
        match k {
            "label" => {
                // Label
                let x = v.as_str().expect("Label cannot be empty.");
                label = String::from(x);
            }
            "source" => {
                // Source
                let x = v.as_str().expect("Source cannot be empty.");
                source = PathBuf::from(x);

                if !source.is_file() {
                    eprintln!("File Note Found: {:?}", source.bright_red());
                }
            }
            "start" => {
                // Start
                let x = v.as_str().expect("Start cannot be empty.");
                start = String::from(x);
            }
            "stop" => {
                // Stop
                let x = v.as_str().expect("Stop cannot be empty.");
                stop = String::from(x);
            }
            "remove_audio" => {
                // Stop
                let x = v
                    .as_bool()
                    .expect("Remove audio must be boolean true or false");
                remove_audio = x;
            }
            "label_position" => {
                // label position

                let x = v.as_str().expect("Start cannot be empty.");
                // println!("Label Position Key Found: {}", x.bright_red());
                match x {
                    "bottom_left" => label_position = LabelPosition::BottomLeft,
                    "bottom_middle" => label_position = LabelPosition::BottomMiddle,
                    "bottom_right" => label_position = LabelPosition::BottomRight,
                    "top_left" => label_position = LabelPosition::TopLeft,
                    "top_middle" => label_position = LabelPosition::TopMiddle,
                    "top_right" => label_position = LabelPosition::TopRight,
                    _ => {
                        eprintln!(
                            "{}: {}",
                            "Received impermissible label position".bright_red(),
                            x.bright_red()
                        )
                    }
                }
            }
            "label_display" => {
                // Stop
                let x = v
                    .as_bool()
                    .expect("Remove audio must be boolean true or false");
                label_display = x;
            }
            _ => {
                eprintln!("{} is not a recognized key.", k.bright_red())
            }
        }
    }

    Clip {
        source,
        label,
        start,
        stop,
        remove_audio,
        label_position,
        label_display,
    }
}

fn read_yaml_file(file: &str) -> Job {
    let yaml = std::fs::read_to_string(file).unwrap();

    println!("MAPPING TO YAML");

    let de = serde_yaml::Deserializer::from_str(&yaml);
    let value = Value::deserialize(de).unwrap();
    // println!("{:?}", value);

    let mut clips_vector: Vec<Clip> = vec![];
    let mut output_directory: Option<PathBuf> = None;

    match value {
        Value::Mapping(map) => {
            for (k, v) in map {
                let k = k.as_str().unwrap();
                match k {
                    "output_directory" => {
                        // Out
                        let x = PathBuf::from_str(v.as_str().unwrap()).unwrap();
                        if !x.is_dir() {
                            // Create directory if it does not already exist
                            let _ = fs::create_dir_all(&x);
                        }
                        output_directory = Some(x);
                    }
                    "clips" => {
                        // Clips
                        if let Some(clips) = v.as_sequence() {
                            for clip in clips {
                                let c = clip.as_mapping().unwrap();
                                let x = make_clip(c);
                                clips_vector.push(x);
                            }
                        }
                    }
                    _ => {
                        eprintln!("{} is not a recognized key.", k.bright_red())
                    }
                }
            }
        }
        _ => {
            eprintln!("Should be map.")
        }
    }

    println!("{} clips found.", clips_vector.len());
    for clip in &clips_vector {
        println!("{:?}", clip.blue());
    }

    if output_directory.is_none() {
        panic!("Output directory must be set");
    }

    Job {
        output_directory: output_directory.unwrap(),
        clips: clips_vector,
    }
}

pub fn cut_clip(
    source: &PathBuf,
    destination: &PathBuf,
    label: &str,
    start: &str,
    stop: &str,
    remove_audio: bool,
    label_position: LabelPosition,
    label_display: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check source file is valid
    if !source.is_file() {
        eprintln!("{}", "Source file is missing.".bright_red());
        panic!("Source file missing.");
    }

    let input_str: &str = source.to_str().expect("Failed to convert path to str.");

    let output_str = destination
        .to_str()
        .expect("Failed to convert path to str.");

    let mut draw_command: String = String::from("[in]");

    // FFMPEG Location Examples:
    // Center :x=(w-text_w)/2:y=(h-text_h)/2
    // Bottom Center :x=(w-text_w)/2:y=(h-text_h)
    // TOP Center :x=(w-text_w):y=(text_h)
    let label_coordinates: String;
    let margin = 5;

    match label_position {
        LabelPosition::BottomLeft => {
            label_coordinates = format!("x=({margin}):y=(h-(text_h+{margin}))")
        }
        LabelPosition::BottomMiddle => {
            label_coordinates = format!("x=(w-text_w)/2:y=(h-(text_h+{margin}))")
        }
        LabelPosition::BottomRight => {
            label_coordinates = format!("x=(w-(text_w+{margin})):y=(h-(text_h+{margin}))")
        }
        LabelPosition::TopLeft => label_coordinates = format!("x=({margin}):y=({margin})"),
        LabelPosition::TopMiddle => label_coordinates = format!("x=(w-text_w)/2:y=({margin})"),
        LabelPosition::TopRight => {
            label_coordinates = format!("x=(w-(text_w+{margin})):y=({margin})")
        }
    }

    //Exhibit Label
    let exhibit_label = format!(
        "\
            drawtext=fontsize=(h/20):
            fontcolor=#38b1fc:
            fontfile='{COURIER_BOLD}':
            text='{label}':
            box=1:\
            boxcolor=Black@0.7:
            boxborderw=5:
            {label_coordinates}
    "
    );
    draw_command.push_str(&exhibit_label);

    draw_command.push_str("[out]");

    let msg = format!("Encoding video clip: {:?}", {
        destination.file_name().unwrap()
    });

    let mut spinner = Spinner::new(spinners::Aesthetic, msg, Color::Yellow);

    let mut start_command = vec!["-i", input_str];
    let mut seek_command = vec!["-ss", start, "-to", stop];

    let mut audio_stream: Vec<&str>;
    if remove_audio {
        // NO AUDIO
        audio_stream = vec!["-an"];
    } else {
        audio_stream = vec!["-c:a", "copy"];
    }
    // let mut gpu_commands = vec!["-c:v", "hevc_nvenc", "-preset", "slow", "-tune", "hq"];
    // let mut _cpu_commands = vec!["-c:v","libx264", "-crf", "18", "-preset", "slow"];
    let mut _cpu_commands = vec!["-c:v", "libx265", "-crf", "20", "-preset", "slow"];

    let mut visual_filters = vec!["-vf", &draw_command];
    let mut output_command = vec!["-y", output_str];

    start_command.append(&mut seek_command);
    start_command.append(&mut audio_stream);
    start_command.append(&mut _cpu_commands);
    if label_display {
        start_command.append(&mut visual_filters);
    }
    start_command.append(&mut output_command);

    let output = if cfg!(target_os = "windows") {
        Command::new("ffmpeg")
            .args(start_command)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo Not Implemented")
            .output()
            .expect("failed to execute process")
    };

    if output.status.success() {
        // println!("Successfully labeled image!");
        spinner.success("Encode completed!");
        Ok(())
    } else {
        let e = String::from_utf8_lossy(&output.stderr).to_string();
        let e = format!("FFMPEG external process failed: {}", e);
        eprintln!("{}", &e.bright_red());
        Err(e)?
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cut() {
        let input_path = PathBuf::from(r".\test\a.mp4");
        let output_path: PathBuf = PathBuf::from(r".\test\output\a.mp4");
        let label = "Ex. 192 A";
        let start = "00:04:00";
        let stop = "00:04:05.100";

        let _ = cut_clip(
            &input_path,
            &output_path,
            label,
            start,
            stop,
            false,
            LabelPosition::BottomLeft,
            true,
        );
    }

    #[test]
    fn test_parse_yaml() {
        let _ = run_job("./test/example.yaml");
    }
}
