use std::io::{self, Write};
use termion::{color, cursor};

pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let default_pixel = Pixel {
            glyph: ' ',
            fg_color: color::Rgb(0, 0, 0),
            bg_color: color::Rgb(0, 0, 0),
        };
        let pixels = vec![default_pixel; width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, glyph: char, color: color::Rgb) {
        for pixel in &mut self.pixels {
            pixel.glyph = glyph;
            pixel.fg_color = color;
        }
    }

    pub fn write(&mut self, x: usize, y: usize, glyph: char, color: color::Rgb) {
        let idx = y * self.width + x;
        self.pixels[idx].glyph = glyph;
        self.pixels[idx].fg_color = color;
    }

    pub fn write_background(&mut self, x: usize, y: usize, color: color::Rgb) {
        let idx = y * self.width + x;
        self.pixels[idx].bg_color = color;
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        write!(stdout, "{}", cursor::Goto(1, 1))?;

        for (y, line) in self.pixels.chunks_exact(self.width).enumerate() {
            for pixel in line {
                write!(
                    stdout,
                    "{}{}{}",
                    color::Bg(pixel.bg_color),
                    color::Fg(pixel.fg_color),
                    pixel.glyph
                )?;
            }

            if y < self.height - 1 {
                write!(stdout, "\r\n")?;
            }
        }

        stdout.flush()?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Pixel {
    glyph: char,
    fg_color: color::Rgb,
    bg_color: color::Rgb,
}
