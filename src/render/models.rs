use glam::f64::DVec3;

pub const VERTICES: [DVec3; 8] = [
    DVec3::new(-1.0, 1.0, 1.0),
    DVec3::new(-1.0, -1.0, 1.0),
    DVec3::new(1.0, -1.0, 1.0),
    DVec3::new(1.0, 1.0, 1.0),
    DVec3::new(1.0, 1.0, -1.0),
    DVec3::new(1.0, -1.0, -1.0),
    DVec3::new(-1.0, -1.0, -1.0),
    DVec3::new(-1.0, 1.0, -1.0),
];

pub const LINES: [usize; 24] = [
    0, 1, 1, 2, 2, 3, 3, 0, // front
    0, 7, 1, 6, 2, 5, 3, 4, // side
    4, 5, 5, 6, 6, 7, 7, 4, // back
];

pub const TRIANGLES: [usize; 36] = [
    0, 1, 2, 0, 2, 3, // front
    3, 2, 5, 3, 5, 4, // right
    7, 0, 3, 7, 3, 4, // top
    1, 6, 5, 1, 5, 2, // bottom
    7, 6, 1, 7, 1, 0, // left
    4, 5, 6, 4, 6, 7, // back
];
