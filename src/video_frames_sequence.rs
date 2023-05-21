use std::{
    io::Read,
    process::{ChildStdout, Command, Stdio},
};

use crate::image_to_ascii_converter::Size;

pub struct VideoFramesSequence<'a> {
    ffmpeg_stdout: ChildStdout,
    size: &'a Size,
}

impl<'a> VideoFramesSequence<'a> {
    pub fn from_video_path(video_path: &'a String, size: &'a Size) -> Self {
        let ffmpeg_stdout = Self::get_ffmpeg_stdout(video_path, size);

        Self {
            ffmpeg_stdout,
            size,
        }
    }

    fn get_ffmpeg_stdout(video_path: &String, size: &Size) -> ChildStdout {
        let Size { width, height } = size;

        Command::new("ffmpeg")
            .args([
                "-i",
                &video_path,
                "-vf",
                format!("scale={width}:{height}").as_str(),
                "-f",
                "image2pipe",
                "-pix_fmt",
                "gray",
                "-vcodec",
                "rawvideo",
                "-nostats",
                "-loglevel",
                "0",
                "-",
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start ffmpeg")
            .stdout
            .unwrap()
    }
}

impl<'a> Iterator for VideoFramesSequence<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut frame_bitmap = vec![0u8; self.size.count_pixels() as usize];
        let bytes_to_read = self.ffmpeg_stdout.read(&mut frame_bitmap).unwrap();

        // The exact number of bytes in stdout is unknown, so i use this hack instad of ChildStdout.read_exact()
        if bytes_to_read == 0 {
            return None;
        }

        Some(frame_bitmap)
    }
}
