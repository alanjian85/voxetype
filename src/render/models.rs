use super::renderer::Vertex;
use glam::f64::{DVec2, DVec3, DVec4};

pub const VERTICES: [Vertex; 24] = [
    // front
    Vertex::new(
        DVec4::new(-1.0, 1.0, 1.0, 1.0),
        DVec2::new(0.0, 1.0),
        DVec3::new(0.0, 0.0, 1.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, -1.0, 1.0, 1.0),
        DVec2::new(0.0, 0.0),
        DVec3::new(0.0, 0.0, 1.0),
    ),
    Vertex::new(
        DVec4::new(1.0, -1.0, 1.0, 1.0),
        DVec2::new(1.0, 0.0),
        DVec3::new(0.0, 0.0, 1.0),
    ),
    Vertex::new(
        DVec4::new(1.0, 1.0, 1.0, 1.0),
        DVec2::new(1.0, 1.0),
        DVec3::new(0.0, 0.0, 1.0),
    ),
    // right
    Vertex::new(
        DVec4::new(1.0, 1.0, 1.0, 1.0),
        DVec2::new(0.0, 1.0),
        DVec3::new(1.0, 0.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, -1.0, 1.0, 1.0),
        DVec2::new(0.0, 0.0),
        DVec3::new(1.0, 0.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, -1.0, -1.0, 1.0),
        DVec2::new(1.0, 0.0),
        DVec3::new(1.0, 0.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, 1.0, -1.0, 1.0),
        DVec2::new(1.0, 1.0),
        DVec3::new(1.0, 0.0, 0.0),
    ),
    // top
    Vertex::new(
        DVec4::new(-1.0, 1.0, -1.0, 1.0),
        DVec2::new(0.0, 1.0),
        DVec3::new(0.0, 1.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, 1.0, 1.0, 1.0),
        DVec2::new(0.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, 1.0, 1.0, 1.0),
        DVec2::new(1.0, 0.0),
        DVec3::new(0.0, 1.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, 1.0, -1.0, 1.0),
        DVec2::new(1.0, 1.0),
        DVec3::new(0.0, 1.0, 0.0),
    ),
    // bottom
    Vertex::new(
        DVec4::new(-1.0, -1.0, 1.0, 1.0),
        DVec2::new(0.0, 1.0),
        DVec3::new(0.0, -1.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, -1.0, -1.0, 1.0),
        DVec2::new(0.0, 0.0),
        DVec3::new(0.0, -1.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, -1.0, -1.0, 1.0),
        DVec2::new(1.0, 0.0),
        DVec3::new(0.0, -1.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(1.0, -1.0, 1.0, 1.0),
        DVec2::new(1.0, 1.0),
        DVec3::new(0.0, -1.0, 0.0),
    ),
    // left
    Vertex::new(
        DVec4::new(-1.0, 1.0, -1.0, 1.0),
        DVec2::new(0.0, 1.0),
        DVec3::new(-1.0, 0.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, -1.0, -1.0, 1.0),
        DVec2::new(0.0, 0.0),
        DVec3::new(-1.0, 0.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, -1.0, 1.0, 1.0),
        DVec2::new(1.0, 0.0),
        DVec3::new(-1.0, 0.0, 0.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, 1.0, 1.0, 1.0),
        DVec2::new(1.0, 1.0),
        DVec3::new(-1.0, 0.0, 0.0),
    ),
    // back
    Vertex::new(
        DVec4::new(1.0, 1.0, -1.0, 1.0),
        DVec2::new(0.0, 1.0),
        DVec3::new(0.0, 0.0, -1.0),
    ),
    Vertex::new(
        DVec4::new(1.0, -1.0, -1.0, 1.0),
        DVec2::new(0.0, 0.0),
        DVec3::new(0.0, 0.0, -1.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, -1.0, -1.0, 1.0),
        DVec2::new(1.0, 0.0),
        DVec3::new(0.0, 0.0, -1.0),
    ),
    Vertex::new(
        DVec4::new(-1.0, 1.0, -1.0, 1.0),
        DVec2::new(1.0, 1.0),
        DVec3::new(0.0, 0.0, -1.0),
    ),
];

pub const TRIANGLES: [usize; 36] = [
    0, 1, 2, 0, 2, 3, // front
    4, 5, 6, 4, 6, 7, // right
    8, 9, 10, 8, 10, 11, // top
    12, 13, 14, 12, 14, 15, // bottom
    16, 17, 18, 16, 18, 19, // left
    20, 21, 22, 20, 22, 23, // back
];
