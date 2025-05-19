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
            clear_value: Pixel::new(' ', color::Rgb(0, 0, 0)),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(self.clear_value);
    }

    pub fn draw_point(&mut self, pos: (f64, f64), pixel: Pixel) {
        let width = self.framebuffer.width() as i32;
        let height = self.framebuffer.height() as i32;

        let (x, y) = self.screen_to_viewport(pos);
        if !(0..width).contains(&x) || !(0..height).contains(&y) {
            return;
        }

        self.framebuffer.write(x as usize, y as usize, pixel);
    }

    pub fn draw_line(&mut self, a: (f64, f64), b: (f64, f64), pixel: Pixel) {
        let a = self.clip(self.screen_to_viewport(a));
        let b = self.clip(self.screen_to_viewport(b));

        let dx = (a.0 - b.0).abs();
        let dy = -(a.1 - b.1).abs();
        let sx = if a.0 < b.0 { 1 } else { -1 };
        let sy = if a.1 < b.1 { 1 } else { -1 };

        let (mut x, mut y) = a;
        let mut err = dx + dy;

        loop {
            self.framebuffer.write(x as usize, y as usize, pixel);
            if (x, y) == b {
                break;
            }

            let err2 = err * 2;
            if err2 >= dy {
                err += dy;
                x += sx;
            }
            if err2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.framebuffer.present(stdout)
    }

    fn screen_to_viewport(&self, (x, y): (f64, f64)) -> (i32, i32) {
        let x = ((x * 0.5 + 0.5) * self.framebuffer.width() as f64).round() as i32;
        let y = ((-y * 0.5 + 0.5) * self.framebuffer.height() as f64).round() as i32;
        (x, y)
    }

    fn clip(&self, (x, y): (i32, i32)) -> (i32, i32) {
        let x = x.clamp(0, (self.framebuffer.width() - 1) as i32);
        let y = y.clamp(0, (self.framebuffer.height() - 1) as i32);
        (x, y)
    }
}
