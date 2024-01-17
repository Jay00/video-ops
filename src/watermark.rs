use crate::utils::COURIER_BOLD;
use owo_colors::OwoColorize;
use spinoff::{spinners, Color, Spinner};
use std::path::PathBuf;
use std::process::Command;

fn watermark_video(
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

    //ID Number
    let id_number_draw = format!(
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
        COURIER_BOLD, id_number
    );
    draw_command.push_str(&id_number_draw);

    // PTS
    draw_command.push_str(",");
    let frame_number_draw = format!(
        r"
            drawtext=fontsize=27:
            fontcolor=#00ff37:
            fontfile='{}':
            start_number=0:
            box=1:
            boxcolor=Black@0.7:
            boxborderw=4:
            text='PTS\:%{{pts \: hms}}':
            x=(w-text_w)/2:
            y=(text_h+20)
        ",
        COURIER_BOLD
    );
    draw_command.push_str(&frame_number_draw);

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
        COURIER_BOLD, id_number
    );

    // If you want frame type: P, I, B
    // text='#{} Frame\:%{{frame_num}} Pict Type\:%{{pict_type}}':

    draw_command.push_str(&frame_number_draw);

    draw_command.push_str("[out]");

    let msg = format!("Encoding video file: {:?}", { source.file_name().unwrap() });

    let mut spinner = Spinner::new(spinners::Aesthetic, msg, Color::Yellow);

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

        let _ = watermark_video(&input_path, &output_path, 295);
    }
}
