use crate::framebuffer::{Framebuffer, Pixel};
use glam::{DMat4, DVec3, Vec3Swizzles, f64::DVec2, i32::IVec2};
use std::io::{self, Write};
use termion::color;

pub struct Renderer {
    framebuffer: Framebuffer,
    transform_mat: DMat4,
    vertex_buf: Vec<Vertex>,
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

    pub fn set_vertex_buf(&mut self, vertex_buf: Vec<Vertex>) {
        self.vertex_buf = vertex_buf;
    }

    pub fn draw_triangles(&mut self, indices: &[usize], symbol: char) {
        for chunk in indices.chunks_exact(3) {
            let vert_a = self.vertex_buf[chunk[0]];
            let vert_b = self.vertex_buf[chunk[1]];
            let vert_c = self.vertex_buf[chunk[2]];

            let a = self.screen_to_viewport(self.transform_mat.project_point3(vert_a.pos).xy());
            let b = self.screen_to_viewport(self.transform_mat.project_point3(vert_b.pos).xy());
            let c = self.screen_to_viewport(self.transform_mat.project_point3(vert_c.pos).xy());

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

                    if det_a >= 0 && det_b >= 0 && det_c >= 0 {
                        let area = edge_a.perp_dot(-edge_c);
                        let uv = (vert_a.uv * det_a as f64
                            + vert_b.uv * det_b as f64
                            + vert_c.uv * det_c as f64)
                            / area as f64;

                        let r = (uv.x * 255.0).round() as u8;
                        let g = (uv.y * 255.0).round() as u8;
                        let b = 0;
                        self.framebuffer.write(
                            x as usize,
                            y as usize,
                            Pixel::new(symbol, color::Rgb(r, g, b)),
                        );
                    }
                }
            }
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

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: DVec3,
    pub uv: DVec2,
}

impl Vertex {
    pub const fn new(pos: DVec3, uv: DVec2) -> Self {
        Self { pos, uv }
    }
}
