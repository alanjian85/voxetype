use crate::framebuffer::{Framebuffer, Pixel};
use std::io::{self, Write};
use termion::color;

pub struct Renderer {
    framebuffer: Framebuffer,
    clear_value: Pixel,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer) -> Self {
        Self {
            framebuffer,
            clear_value: Pixel(' ', color::Rgb(0, 0, 0)),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(self.clear_value);
    }

    pub fn draw_point(&mut self, (x, y): (usize, usize), pixel: Pixel) {
        self.framebuffer.write(x, y, pixel);
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.framebuffer.present(stdout)
    }
}
