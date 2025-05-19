use crate::framebuffer::{Framebuffer, Pixel};
use glam::{f64::DVec2, i32::IVec2};
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

    pub fn draw_point(&mut self, pos: DVec2, pixel: Pixel) {
        let pos = self.screen_to_viewport(pos);

        let width = self.framebuffer.width() as i32;
        let height = self.framebuffer.height() as i32;
        if !(0..width).contains(&pos.x) || !(0..height).contains(&pos.y) {
            return;
        }

        self.framebuffer
            .write(pos.x as usize, pos.y as usize, pixel);
    }

    pub fn draw_line(&mut self, a: DVec2, b: DVec2, pixel: Pixel) {
        let a = self.viewport_clip(self.screen_to_viewport(a));
        let b = self.viewport_clip(self.screen_to_viewport(b));

        let dx = (a.x - b.x).abs();
        let dy = -(a.y - b.y).abs();
        let sx = if a.x < b.x { 1 } else { -1 };
        let sy = if a.y < b.y { 1 } else { -1 };

        let mut pos = a;
        let mut err = dx + dy;

        loop {
            self.framebuffer
                .write(pos.x as usize, pos.y as usize, pixel);
            if pos == b {
                break;
            }

            let err2 = err * 2;
            if err2 >= dy {
                err += dy;
                pos.x += sx;
            }
            if err2 <= dx {
                err += dx;
                pos.y += sy;
            }
        }
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.framebuffer.present(stdout)
    }

    fn screen_to_viewport(&self, v: DVec2) -> IVec2 {
        ((v * DVec2::new(0.5, -0.5) + 0.5)
            * DVec2::new(
                self.framebuffer.width() as f64,
                self.framebuffer.height() as f64,
            )
            .round())
        .as_ivec2()
    }

    fn viewport_clip(&self, v: IVec2) -> IVec2 {
        v.clamp(
            IVec2::new(0, 0),
            IVec2::new(
                (self.framebuffer.width() - 1) as i32,
                (self.framebuffer.height() - 1) as i32,
            ),
        )
    }
}
