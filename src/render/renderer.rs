use crate::framebuffer::{Framebuffer, Pixel};
use glam::{DMat4, DVec3, DVec4, Vec3Swizzles, Vec4Swizzles, f64::DVec2, i32::IVec2};
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

    pub fn draw_triangles<F>(&mut self, indices: &[usize], sample_texture: F)
    where
        F: Fn(DVec2) -> Pixel,
    {
        for chunk in indices.chunks_exact(3) {
            let vert_a = self.vertex_buf[chunk[0]];
            let vert_b = self.vertex_buf[chunk[1]];
            let vert_c = self.vertex_buf[chunk[2]];

            let a = self.transform_mat * DVec4::new(0.0, 0.0, 0.0, 1.0).with_xyz(vert_a.pos);
            let b = self.transform_mat * DVec4::new(0.0, 0.0, 0.0, 1.0).with_xyz(vert_b.pos);
            let c = self.transform_mat * DVec4::new(0.0, 0.0, 0.0, 1.0).with_xyz(vert_c.pos);

            let rwa = 1.0 / a.w;
            let rwb = 1.0 / b.w;
            let rwc = 1.0 / c.w;

            let uva = vert_a.uv * rwa;
            let uvb = vert_b.uv * rwb;
            let uvc = vert_c.uv * rwc;

            let a = self.screen_to_viewport((a * rwa).xy());
            let b = self.screen_to_viewport((b * rwb).xy());
            let c = self.screen_to_viewport((c * rwc).xy());

            let min = a.min(b).min(c);
            let max = a.max(b).max(c);

            let edge_a = c - b;
            let edge_b = a - c;
            let edge_c = b - a;

            for x in min.x..=max.x {
                for y in min.y..=max.y {
                    let p = IVec2::new(x, y);
                    let pa = a - p;
                    let pb = b - p;
                    let pc = c - p;

                    let det_a = edge_a.perp_dot(pb) as f64;
                    let det_b = edge_b.perp_dot(pc) as f64;
                    let det_c = edge_c.perp_dot(pa) as f64;

                    if det_a >= 0.0 && det_b >= 0.0 && det_c >= 0.0 {
                        let area = edge_a.perp_dot(edge_c) as f64;
                        let rw = (rwa * det_a + rwb * det_b + rwc * det_c) / area;
                        let uv = (uva * det_a + uvb * det_b + uvc * det_c) / (area * rw);
                        self.framebuffer
                            .write(x as usize, y as usize, sample_texture(uv));
                    }
                }
            }
        }
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

    pub fn present(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.framebuffer.present(stdout)
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
