extern crate image;

use self::image::imageops::resize;

use crate::braille::ToBraille;

pub struct Display {
    image: image::GrayImage,
    char_width: u32,
    char_height: u32,
}

impl Display {
    pub fn new(img: image::GrayImage, width: u32, height: u32) -> Display {
        Display {
            image: resize(&img, width * 2, height * 4, image::Lanczos3),
            char_width: width,
            char_height: height,
        }
    }

    pub fn render(&self) -> Vec<String> {
        let mut target = Vec::<String>::new();

        for y in 0..self.char_height {
            let mut line = String::new();
            for x in 0..self.char_width {
                line = format!("{}{}", line, self.braillify_block(x, y).to_string());
            }
            target.push(line);
        };
        target
    }

    fn braillify_block(&self, x: u32, y: u32) -> char {
        let mut dot_map = 0b0000_0000;
        for i in 0..8 {
            let abs_x = (x * 2) + (i % 2);
            let abs_y = (y * 4) + (i / 2);

            dot_map |= if self.sample(abs_x, abs_y) {
                0b1000_0000 >> i
            } else {
                0
            };
        }
        dot_map.to_braille()
    }

    fn sample(&self, x: u32, y: u32) -> bool {
        self.image[(x, y)][0] < 128
    }
}

