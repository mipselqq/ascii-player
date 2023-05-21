use std::process::Command;

pub struct AudioPlayer<'a> {
    file_path: &'a String,
}

impl<'a> AudioPlayer<'a> {
    pub fn from_file_path(file_path: &'a String) -> Self {
        AudioPlayer { file_path }
    }

    pub fn play(&self) -> () {
        Command::new("ffplay")
            .args([
                "-vn",
                "-nodisp",
                "-loglevel",
                "0",
                "-nostats",
                self.file_path,
            ])
            .spawn()
            .expect("Failed to execute ffplay command");
    }
}
