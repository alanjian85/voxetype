use crate::framebuffer::Framebuffer;
use glam::{DMat4, DVec3, DVec4, Vec4Swizzles, f64::DVec2, i32::IVec2};
use std::io::{self, Write};
use termion::color;

pub struct Renderer {
    framebuffer: Framebuffer,
    transform_mat: DMat4,
    normal_mat: DMat4,
    vertex_buf: Vec<Vertex>,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer) -> Self {
        Self {
            framebuffer,
            transform_mat: DMat4::IDENTITY,
            normal_mat: DMat4::IDENTITY,
            vertex_buf: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(' ', color::Rgb(0, 0, 0));
    }

    pub fn set_transform_mat(&mut self, transform_mat: DMat4) {
        self.transform_mat = transform_mat;
    }

    pub fn set_model_mat(&mut self, model_mat: DMat4) {
        self.normal_mat = model_mat.inverse().transpose();
    }

    pub fn set_vertex_buf(&mut self, vertex_buf: Vec<Vertex>) {
        self.vertex_buf = vertex_buf;
    }

    pub fn draw_triangles<F>(&mut self, indices: &[usize], shader: F)
    where
        F: Fn(DVec2, DVec3) -> (char, color::Rgb),
    {
        for chunk in indices.chunks_exact(3) {
            let vert_a = self.vertex_buf[chunk[0]];
            let vert_b = self.vertex_buf[chunk[1]];
            let vert_c = self.vertex_buf[chunk[2]];

            let a = self.transform_mat * DVec4::new(0.0, 0.0, 0.0, 1.0).with_xyz(vert_a.pos);
            let b = self.transform_mat * DVec4::new(0.0, 0.0, 0.0, 1.0).with_xyz(vert_b.pos);
            let c = self.transform_mat * DVec4::new(0.0, 0.0, 0.0, 1.0).with_xyz(vert_c.pos);

            let rw_a = 1.0 / a.w;
            let rw_b = 1.0 / b.w;
            let rw_c = 1.0 / c.w;

            let uv_a = vert_a.uv * rw_a;
            let uv_b = vert_b.uv * rw_b;
            let uv_c = vert_c.uv * rw_c;

            let normal_a = vert_a.normal * rw_a;
            let normal_b = vert_b.normal * rw_b;
            let normal_c = vert_c.normal * rw_c;

            let a = self.screen_to_viewport((a * rw_a).xy());
            let b = self.screen_to_viewport((b * rw_b).xy());
            let c = self.screen_to_viewport((c * rw_c).xy());

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
                        let rw = (rw_a * det_a + rw_b * det_b + rw_c * det_c) / area;
                        let uv = (uv_a * det_a + uv_b * det_b + uv_c * det_c) / (area * rw);
                        let normal = self
                            .normal_mat
                            .transform_vector3(
                                normal_a * det_a + normal_b * det_b + normal_c * det_c,
                            )
                            .normalize();
                        let (glyph, color) = shader(uv, normal);
                        self.framebuffer.write(x as usize, y as usize, glyph, color);
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
    pub normal: DVec3,
}

impl Vertex {
    pub const fn new(pos: DVec3, uv: DVec2, normal: DVec3) -> Self {
        Self { pos, uv, normal }
    }
}
