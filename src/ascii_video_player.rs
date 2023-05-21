use std::time::{Duration, Instant};

use crate::{
    ffprobe_video_stream::FfprobeVideoStream,
    image_to_ascii_converter::{ImageToAsciiConverter, Size},
    video_frames_sequence::VideoFramesSequence,
};

pub struct AsciiVideoPlayer<'a> {
    video_path: &'a String,
}

impl<'a> AsciiVideoPlayer<'a> {
    pub fn from_video_path(video_path: &'a String) -> Self {
        Self { video_path }
    }

    pub fn play(&self) -> () {
        let ffprobe_stream = FfprobeVideoStream::from_file_path(&self.video_path);
        let aspect_ratio = ffprobe_stream.parse_aspect_ratio();
        let framerate = ffprobe_stream.parse_framerate();

        let size = &Self::calculate_frame_size(aspect_ratio);
        let video_sequence = VideoFramesSequence::from_video_path(&self.video_path, size);

        let frame_duration = Duration::from_secs_f32(1.0 / framerate as f32);

        for frame_bitmap in video_sequence {
            let image_to_ascii_converter = ImageToAsciiConverter::new(&frame_bitmap, size);

            println!("{}", image_to_ascii_converter.convert());

            let frame_time = Instant::now();
            let elapsed_time = frame_time.elapsed();

            if elapsed_time < frame_duration {
                std::thread::sleep(frame_duration - elapsed_time);
            }
        }
    }

    fn calculate_frame_size(aspect_ratio: f32) -> Size {
        let (_, terminal_height) = term_size::dimensions().unwrap();

        let height = terminal_height as u32;
        let width_correction = 1.8;
        let width = ((height as f32 * aspect_ratio) * width_correction) as u32;

        Size { width, height }
    }
}
