use crate::utils::courier_bold;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::process::Command;

fn cut_clip(
    source: &PathBuf,
    destination: &PathBuf,
    id_number: usize,
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
            fontfile='{}':
            text='#{}':
            box=1:\
            boxcolor=Black@0.7:
            boxborderw=5:
            x=(w-text_w)/2:
            y=(5)
    ",
        courier_bold, id_number
    );
    draw_command.push_str(&exhibit_label);

    // Frames
    draw_command.push_str(",");
    let frame_number_draw = format!(
        r"
            drawtext=fontsize=20:
            fontcolor=#00ff37:
            fontfile='{}':
            start_number=0:
            box=1:
            boxcolor=Black@0.7:
            boxborderw=4:
            text='#{} Frame\:%{{frame_num}}':
            x=5:
            y=(h-text_h-5)
        ",
        courier_bold, id_number
    );

    // If you want frame type: P, I, B
    // text='#{} Frame\:%{{frame_num}} Pict Type\:%{{pict_type}}':

    draw_command.push_str(&frame_number_draw);

    draw_command.push_str("[out]");

    let msg = format!("Encoding video file: {:?}", { source.file_name().unwrap() });

    let spinner = spinoff::Spinner::new(spinners::Aesthetic, msg, Color::Yellow);

    let output = if cfg!(target_os = "windows") {
        Command::new("ffmpeg")
            .args([
                "-i",
                input_str,
                "-c:a",
                "copy",
                "-c:v",
                "hevc_nvenc",
                // "-crf",
                // "26", //28  0-51
                // HEVC NVENC settings
                "-preset",
                "slow",
                "-tune",
                "hq",
                // "-profile:v",
                // "main10",
                //
                "-vf",
                &draw_command,
                // "-r",           // set the frame rate to convert variable frame rates to constant
                // avg_frame_rate, // should make the timecode display match the counter
                "-y",
                output_str,
            ])
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

        let _ = cut_clip(&input_path, &output_path, 295);
    }
}
