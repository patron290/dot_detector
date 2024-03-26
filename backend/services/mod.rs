extern crate ffmpeg_next as ffmpeg;

use ffmpeg::{format, frate, media::Type, software::scaling::{self, flag::Flags}};
use std::path::Path;

async fn save_video_frames_as_png(video_path: &Path) -> Result<(), ffmpeg::Error> {
    ffmpeg::init()?;

    let mut ictx = format::input(&video_path)?;
    let input_stram = ictx.streams().best(Type::Video).expect("No video stream found");
}