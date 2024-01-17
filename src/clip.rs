use crate::utils::courier_bold;
use owo_colors::OwoColorize;
use spinoff::{spinners, Color, Spinner};
use std::path::PathBuf;
use std::process::Command;

fn cut_clip(
    source: &PathBuf,
    destination: &PathBuf,
    label: &str,
    start: &str,
    stop: &str,
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

    //Exhibit Label
    let exhibit_label = format!(
        "\
            drawtext=fontsize=30:
            fontcolor=#00ff37:
            fontfile='{courier_bold}':
            text='{label}':
            box=1:\
            boxcolor=Black@0.7:
            boxborderw=5:
            x=(w-text_w)/2:
            y=(5)
    "
    );
    draw_command.push_str(&exhibit_label);

    draw_command.push_str("[out]");

    let msg = format!("Encoding video file: {:?}", { source.file_name().unwrap() });

    let mut spinner = Spinner::new(spinners::Aesthetic, msg, Color::Yellow);

    let mut start_command = vec!["-i", input_str];
    let seek_command = vec!["-ss", start, "-to", stop];
    let streams_command = vec!["-c:a", "copy", "-c:v"];
    let gpu_commands = vec!["hevc_nvenc", "-preset", "slow", "-tune", "hq"];
    let _cpu_commands = vec!["libx265", "-crf", "26", "-preset", "slow"];
    let visual_filters = vec!["-vf", &draw_command];
    let output_command = vec!["-y", output_str];

    start_command.extend(seek_command);
    start_command.extend(streams_command);
    start_command.extend(gpu_commands);
    start_command.extend(visual_filters);
    start_command.extend(output_command);

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
    fn test_draw() {
        let input_path = PathBuf::from(r".\testvideo\b.mp4");
        let output_path: PathBuf = PathBuf::from(r".\testvideo\out\b.mp4");
        let label = "Ex. 192 A";
        let start = "00:00:01";
        let stop = "00:00:05";

        let _ = cut_clip(&input_path, &output_path, label, start, stop);
    }
}
