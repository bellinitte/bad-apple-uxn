use ffmpeg_next as ffmpeg;
use image::GrayImage;
use anyhow::{anyhow, Result};

pub struct Decoder<'a> {
    packets: ffmpeg::format::context::input::PacketIter<'a>,
    video_stream_index: usize,
    decoder: ffmpeg::decoder::Video,
    scaler: ffmpeg::software::scaling::Context,
}

pub trait Decode<'a> {
    fn decode(&'a mut self) -> Result<Decoder<'a>>;
}

impl<'a> Decode<'a> for ffmpeg::format::context::Input {
    fn decode(&'a mut self) -> Result<Decoder<'a>>{
        Decoder::new(self)
    }
}

impl<'a> Decoder<'a> {
    pub(self) fn new(input: &'a mut ffmpeg::format::context::Input) -> Result<Self> {
        let input_stream = input
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input_stream.index();

        let decoder = input_stream.codec().decoder().video()?;

        let scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg::format::Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            ffmpeg::software::scaling::Flags::BILINEAR,
        )?;

        Ok(Self {
            packets: input.packets(),
            video_stream_index,
            decoder,
            scaler,
        })
    }
}

impl<'a> Iterator for Decoder<'a> {
    type Item = Result<GrayImage>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(frame) = self.receive_decoded_frames().transpose() {
            return Some(frame);
        }

        while let Some((stream, packet)) = self.packets.next() {
            if stream.index() == self.video_stream_index {
                if let Err(error) = self.decoder.send_packet(&packet) {
                    return Some(Err(anyhow!(error)));
                }
                if let Some(frame) = self.receive_decoded_frames().transpose() {
                    return Some(frame);
                }
            }
        }

        if self.decoder.send_eof().is_ok() {
            return self.receive_decoded_frames().transpose();
        }
        
        None
    }
}

impl<'a> Decoder<'a> {
    fn receive_decoded_frames(&mut self) -> Result<Option<GrayImage>> {
        let mut decoded = ffmpeg::util::frame::video::Video::empty();

        if self.decoder.receive_frame(&mut decoded).is_err() {
            return Ok(None);
        }

        let mut rgb_frame = ffmpeg::util::frame::video::Video::empty();
        self.scaler.run(&decoded, &mut rgb_frame)?;

        let data: Vec<u8> = rgb_frame
            .data(0)
            .chunks(3)
            .map(|chunk| chunk[0])
            .collect();
        
        Ok(GrayImage::from_raw(
            rgb_frame.width(),
            rgb_frame.height(),
            data,
        ))
    }
}
