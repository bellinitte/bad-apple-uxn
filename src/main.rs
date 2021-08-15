use encoder::encode;
use ffmpeg_next as ffmpeg;
use anyhow::Result;
use decoder::Decode;
use image::{GrayImage, Pixel, imageops::{resize, FilterType}};
use std::path::PathBuf;
use structopt::StructOpt;
use ffmpeg::format::input;
use indicatif::ProgressBar;
use itertools::Itertools;

mod decoder;
mod encoder;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "v", long = "video")]
    video_path: PathBuf,
    #[structopt(short = "o", long = "output")]
    binary_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    ffmpeg::init().unwrap();

    let progress_bar = ProgressBar::new(6571 / 8 as u64);

    let _ = input(&args.video_path)?
        .decode()?
        .step_by(8)
        .map_ok(|frame| {
            let frame = resize(&frame, 42, 32, FilterType::Triangle);
            frame
        })
        .map_ok(|frame: GrayImage| {
            frame.pixels().map(|pixel| pixel.channels()[0]).map(|value| value >= 128).collect::<Vec<bool>>()
        })
        .map_ok(|frame: Vec<bool>| encode(frame))
        .inspect(|_| progress_bar.inc(1))
        .inspect(|frame| {
            if let Ok(frame) = frame {
                println!("{} bytes", frame.len());
            }
        })
        .collect::<Result<Vec<Vec<u8>>>>()?;

    progress_bar.finish();

    Ok(())
}
