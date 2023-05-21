pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn count_pixels(&self) -> u32 {
        self.width * self.height
    }
}

pub struct ImageToAsciiConverter<'a> {
    pub bitmap: &'a [u8],
    pub size: &'a Size,
    pub chars: [char; 13],
    pub brightness_interval: u32,
}

impl<'a> ImageToAsciiConverter<'a> {
    pub fn new(bitmap: &'a [u8], size: &'a Size) -> Self {
        let chars = [
            ' ', '`', '·', ':', '-', '~', '=', '+', '*', '#', '%', '@', 'Ø',
        ];

        let brightness_interval = Self::calculate_brightness_interval(&chars);

        ImageToAsciiConverter {
            bitmap,
            size,
            chars,
            brightness_interval,
        }
    }

    pub fn convert(&self) -> String {
        let width = self.size.width;
        let pixel_count = self.size.count_pixels();
        let mut buffer = String::with_capacity(pixel_count as usize);

        for i in 0..pixel_count {
            let brightness = self.bitmap[i as usize] as u32;
            let char = self.convert_single_pixel(brightness);

            buffer.push(char);

            if i % width == width - 1 {
                buffer.push('\n');
            }
        }

        buffer
    }

    pub fn convert_single_pixel(&self, brightness: u32) -> char {
        let char_index = brightness / self.brightness_interval;

        self.chars[char_index as usize]
    }

    pub fn calculate_brightness_interval(chars: &[char]) -> u32 {
        let chars_count = chars.len();

        255 / (chars_count - 1) as u32
    }
}
