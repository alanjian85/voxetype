use std::io::{self, Write};
use termion::{color, cursor};

pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

#[derive(Clone)]
pub struct Pixel(pub char, pub color::Rgb);

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Pixel(' ', color::Rgb(0, 0, 0)); width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write(&mut self, x: usize, y: usize, pixel: Pixel) {
        let idx = y * self.width + x;
        self.pixels[idx] = pixel;
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        write!(stdout, "{}", cursor::Goto(1, 1))?;

        for (y, line) in self.pixels.chunks_exact(self.width).enumerate() {
            for pixel in line {
                write!(stdout, "{}{}", color::Fg(pixel.1), pixel.0)?;
            }

            if y < self.height - 1 {
                write!(stdout, "\r\n")?;
            }
        }

        Ok(())
    }
}
