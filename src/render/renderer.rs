use crate::framebuffer::Framebuffer;
use glam::{DVec3, DVec4, Vec4Swizzles, f64::DVec2, i32::IVec2};
use std::{
    io::{self, Write},
    ops::{Add, Div, Mul},
};
use termion::color;

pub struct Renderer {
    framebuffer: Framebuffer,
    vertex_buf: Vec<Vertex>,
}

impl Renderer {
    pub fn new(framebuffer: Framebuffer) -> Self {
        Self {
            framebuffer,
            vertex_buf: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(' ', color::Rgb(0, 0, 0));
    }

    pub fn set_vertex_buf(&mut self, vertex_buf: Vec<Vertex>) {
        self.vertex_buf = vertex_buf;
    }

    pub fn draw_triangles<V, F>(&mut self, indices: &[usize], vert_shader: &V, frag_shader: &F)
    where
        V: Fn(Vertex) -> Vertex,
        F: Fn(Vertex) -> (char, color::Rgb),
    {
        for chunk in indices.chunks_exact(3) {
            let vert_a = vert_shader(self.vertex_buf[chunk[0]]);
            let vert_b = vert_shader(self.vertex_buf[chunk[1]]);
            let vert_c = vert_shader(self.vertex_buf[chunk[2]]);

            let rw_a = 1.0 / vert_a.pos.w;
            let rw_b = 1.0 / vert_b.pos.w;
            let rw_c = 1.0 / vert_c.pos.w;

            let vert_a = vert_a * rw_a;
            let vert_b = vert_b * rw_b;
            let vert_c = vert_c * rw_c;

            let a = self.screen_to_viewport(vert_a.pos.xy());
            let b = self.screen_to_viewport(vert_b.pos.xy());
            let c = self.screen_to_viewport(vert_c.pos.xy());

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
                        let denom = rw_a * det_a + rw_b * det_b + rw_c * det_c;
                        let vert = (vert_a * det_a + vert_b * det_b + vert_c * det_c) / denom;
                        let (glyph, color) = frag_shader(vert);
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
    pub pos: DVec4,
    pub uv: DVec2,
    pub normal: DVec3,
}

impl Vertex {
    pub const fn new(pos: DVec4, uv: DVec2, normal: DVec3) -> Self {
        Self { pos, uv, normal }
    }
}

impl Add<Vertex> for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            pos: self.pos + rhs.pos,
            uv: self.uv + rhs.uv,
            normal: self.normal + rhs.normal,
        }
    }
}

impl Mul<f64> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            pos: self.pos * rhs,
            uv: self.uv * rhs,
            normal: self.normal * rhs,
        }
    }
}

impl Div<f64> for Vertex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            pos: self.pos / rhs,
            uv: self.uv / rhs,
            normal: self.normal / rhs,
        }
    }
}
