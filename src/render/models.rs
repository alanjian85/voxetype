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

pub const LINES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (0, 7),
    (1, 6),
    (2, 5),
    (3, 4),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
];
