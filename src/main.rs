mod ascii_video_player;
mod audio_player;
mod ffprobe_video_stream;
mod image_to_ascii_converter;
mod video_frames_sequence;

use std::env;

use ascii_video_player::AsciiVideoPlayer;
use audio_player::AudioPlayer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let video_path = &args[1];

    let ascii_video_player = AsciiVideoPlayer::from_video_path(video_path);
    let audio = AudioPlayer::from_file_path(video_path);

    audio.play();
    ascii_video_player.play();
}
