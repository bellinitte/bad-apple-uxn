use anyhow::Result;
use decoder::Decode;
use encoder::encode;
use ffmpeg::format::input;
use ffmpeg_next as ffmpeg;
use image::{
    imageops::{resize, FilterType},
    GrayImage, Pixel,
};
use indicatif::ProgressBar;
use itertools::Itertools;
use renderer::{render_uxntal, Value};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod decoder;
mod encoder;
mod renderer;
mod trimmer;

const SCREEN_WIDTH: u16 = 512;
const SCREEN_HEIGHT: u16 = 320;
const VIDEO_WIDTH: u16 = 42;
const VIDEO_HEIGHT: u16 = 32;
const STEP: u16 = 8;
const COLOR_THRESHOLD: u8 = 128;

const X_OFFSET: u16 = (SCREEN_WIDTH - FRAGMENT_WIDTH * VIDEO_WIDTH) / 2;
const Y_OFFSET: u16 = (SCREEN_HEIGHT - FRAGMENT_HEIGHT * VIDEO_HEIGHT) / 2;
const FRAGMENT_WIDTH: u16 = FRAGMENT_HEIGHT;
const FRAGMENT_HEIGHT: u16 = SCREEN_HEIGHT / VIDEO_HEIGHT;
const INPUT_FRAMES_ESTIMATE: u64 = 6571 / STEP as u64;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "i", long = "input")]
    video_path: PathBuf,
    #[structopt(short = "o", long = "output")]
    uxntal_path: PathBuf,
    #[structopt(short = "r", long = "rom")]
    rom_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    ffmpeg::init().unwrap();

    let progress_bar = ProgressBar::new(INPUT_FRAMES_ESTIMATE);

    let encoded_frames = input(&args.video_path)?
        .decode()?
        .step_by(STEP as usize)
        .map_ok(|frame| {
            let frame = resize(
                &frame,
                VIDEO_WIDTH as u32,
                VIDEO_HEIGHT as u32,
                FilterType::Triangle,
            );
            frame
        })
        .map_ok(|frame: GrayImage| {
            frame
                .pixels()
                .map(|pixel| pixel.channels()[0])
                .map(|value| value >= COLOR_THRESHOLD)
                .collect::<Vec<bool>>()
        })
        .map_ok(|frame: Vec<bool>| encode(frame))
        .inspect(|_| progress_bar.inc(1))
        .collect::<Result<Vec<Vec<u8>>>>()?;

    progress_bar.finish();

    let encoded_frames =
        trimmer::trim_frames(encoded_frames, VIDEO_WIDTH as usize * VIDEO_HEIGHT as usize);

    let tweakables: Vec<(&'static str, Value)> = vec![
        ("X-OFFSET", X_OFFSET.into()),
        ("Y-OFFSET", Y_OFFSET.into()),
        ("FRAGMENT-WIDTH", FRAGMENT_WIDTH.into()),
        ("FRAGMENT-HEIGHT", FRAGMENT_HEIGHT.into()),
        ("VIDEO-WIDTH", VIDEO_WIDTH.into()),
        ("VIDEO-HEIGHT", VIDEO_HEIGHT.into()),
        ("FRAME-TIME", (STEP * 2).into()), // 60 FPS Varvara screen vs 30 FPS input video
        ("STOP-TIME", (encoded_frames.len() as u16).into()),
    ];

    let source_string = render_uxntal(encoded_frames, tweakables);

    fs::write(args.uxntal_path, &source_string)?;

    if let Some(rom_path) = args.rom_path {
        let binary = ruxnasm::assemble(&source_string).unwrap().0;

        fs::write(rom_path, &binary)?;
    }

    Ok(())
}
