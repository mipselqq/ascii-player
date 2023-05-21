pub struct FfprobeVideoStream {
    internal_stream: ffprobe::Stream,
}

impl FfprobeVideoStream {
    pub fn from_file_path(file_path: &String) -> Self {
        let ffprobe_output = ffprobe::ffprobe(file_path).expect("Failed to start ffprobe");
        let internal_stream = ffprobe_output.streams.get(0).unwrap();

        Self {
            internal_stream: internal_stream.clone(),
        }
    }

    pub fn parse_framerate(&self) -> f32 {
        let fraction: Vec<f32> = self
            .internal_stream
            .avg_frame_rate
            .split('/')
            .map(|part| part.parse::<f32>().expect("Failed to parse framerate"))
            .collect();

        let numerator = fraction[0];
        let denominator = fraction[1];

        return numerator / denominator;
    }

    pub fn parse_aspect_ratio(&self) -> f32 {
        let aspects: Vec<f32> = self
            .internal_stream
            .display_aspect_ratio
            .as_ref()
            .unwrap()
            .split(':')
            .map(|part| part.parse::<f32>().expect("Failed to parse aspect ratio"))
            .collect();

        let width = aspects[0];
        let height = aspects[1];

        width / height
    }
}
