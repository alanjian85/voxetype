use crate::framebuffer::{Framebuffer, Pixel};
use glam::{DMat4, DVec3, Vec3Swizzles, f64::DVec2, i32::IVec2};
use std::io::{self, Write};
use termion::color;

pub struct Renderer {
    framebuffer: Framebuffer,
    transform_mat: DMat4,
    vertex_buf: Vec<DVec3>,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer) -> Self {
        Self {
            framebuffer,
            transform_mat: DMat4::IDENTITY,
            vertex_buf: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(Pixel::new(' ', color::Rgb(0, 0, 0)));
    }

    pub fn set_transform_mat(&mut self, transform_mat: DMat4) {
        self.transform_mat = transform_mat;
    }

    pub fn set_vertex_buf(&mut self, vertex_buf: Vec<DVec3>) {
        self.vertex_buf = vertex_buf;
    }

    pub fn draw_point(&mut self, pos: DVec3, pixel: Pixel) {
        let pos = self.screen_to_viewport(self.transform_mat.project_point3(pos).xy());

        let width = self.framebuffer.width() as i32;
        let height = self.framebuffer.height() as i32;
        if !(0..width).contains(&pos.x) || !(0..height).contains(&pos.y) {
            return;
        }

        self.framebuffer
            .write(pos.x as usize, pos.y as usize, pixel);
    }

    pub fn draw_line(&mut self, a: DVec3, b: DVec3, pixel: Pixel) {
        let a = self.screen_to_viewport(self.transform_mat.project_point3(a).xy());
        let b = self.screen_to_viewport(self.transform_mat.project_point3(b).xy());

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

    pub fn draw_triangle(&mut self, a: DVec3, b: DVec3, c: DVec3, pixel: Pixel) {
        let a = self.screen_to_viewport(self.transform_mat.project_point3(a).xy());
        let b = self.screen_to_viewport(self.transform_mat.project_point3(b).xy());
        let c = self.screen_to_viewport(self.transform_mat.project_point3(c).xy());

        let min = a.min(b).min(c);
        let max = a.max(b).max(c);

        let edge_a = c - b;
        let edge_b = a - c;
        let edge_c = b - a;

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let p = IVec2::new(x, y);
                let ap = p - a;
                let bp = p - b;
                let cp = p - c;

                let det_a = edge_a.perp_dot(bp);
                let det_b = edge_b.perp_dot(cp);
                let det_c = edge_c.perp_dot(ap);

                if det_a <= 0 && det_b <= 0 && det_c <= 0 {
                    self.framebuffer.write(x as usize, y as usize, pixel);
                }
            }
        }
    }

    pub fn draw_points_index(&mut self, indices: &[usize], pixel: Pixel) {
        for &index in indices {
            self.draw_point(self.vertex_buf[index], pixel);
        }
    }

    pub fn draw_lines_index(&mut self, indices: &[usize], pixel: Pixel) {
        for chunk in indices.chunks_exact(2) {
            self.draw_line(self.vertex_buf[chunk[0]], self.vertex_buf[chunk[1]], pixel);
        }
    }

    pub fn draw_triangles_index(&mut self, indices: &[usize], pixel: Pixel) {
        for chunk in indices.chunks_exact(3) {
            self.draw_triangle(
                self.vertex_buf[chunk[0]],
                self.vertex_buf[chunk[1]],
                self.vertex_buf[chunk[2]],
                pixel,
            );
        }
    }

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.framebuffer.present(stdout)
    }

    fn screen_to_viewport(&self, v: DVec2) -> IVec2 {
        ((v * DVec2::new(0.5, -0.5) + 0.5)
            * DVec2::new(
                (self.framebuffer.width() - 1) as f64,
                (self.framebuffer.height() - 1) as f64,
            )
            .round())
        .as_ivec2()
    }
}
