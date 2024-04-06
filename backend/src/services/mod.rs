use std::fs;
use std::process::Command;

pub fn save_video_frames_as_png(
    video_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_path)?;

    let output_pattern = format!("{}/frame_%04d.png", output_path);
    Command::new("ffmpeg")
        .args(&["-i", video_path, "-vf", "fps=30", &output_pattern])
        .output()?;

    Ok(())
}
