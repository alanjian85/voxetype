use std::io::{self, Write};
use termion::{color, cursor};

pub struct Framebuffer {
    width: usize,
    height: usize,
    background: color::Rgb,
    pixels: Vec<Pixel>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background: color::Rgb) -> Self {
        let pixels = vec![Pixel::new(' ', color::Rgb(0, 0, 0)); width * height];
        Self {
            width,
            height,
            background,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, pixel: Pixel) {
        self.pixels.fill(pixel);
    }

    pub fn write(&mut self, x: usize, y: usize, pixel: Pixel) {
        let idx = y * self.width + x;
        self.pixels[idx] = pixel;
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        write!(
            stdout,
            "{}{}",
            cursor::Goto(1, 1),
            color::Bg(self.background)
        )?;

        for (y, line) in self.pixels.chunks_exact(self.width).enumerate() {
            for pixel in line {
                write!(stdout, "{}{}", color::Fg(pixel.color), pixel.symbol)?;
            }

            if y < self.height - 1 {
                write!(stdout, "\r\n")?;
            }
        }

        stdout.flush()?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub symbol: char,
    pub color: color::Rgb,
}

impl Pixel {
    pub fn new(symbol: char, color: color::Rgb) -> Self {
        Self { symbol, color }
    }
}
