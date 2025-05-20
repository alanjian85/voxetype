use glam::{DMat4, DVec2, DVec3, Vec3Swizzles};
use std::{
    error::Error,
    f64,
    io::{self, Read, Write},
    time::Instant,
};
use termion::{color, cursor, raw::IntoRawMode};

pub mod framebuffer;
pub mod render;

pub use framebuffer::{Framebuffer, Pixel};
pub use render::{
    Renderer,
    models::{LINES, TRIANGLES, VERTICES},
};

const FULL_BLOCK_WIDTH: usize = 10;
const FULL_BLOCK_HEIGHT: usize = 22;

pub fn run(width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout()
        .lock()
        .into_raw_mode()
        .expect("unable to switch stdout to raw mode");
    let mut stdin = termion::async_stdin();
    write!(stdout, "{}", cursor::Hide)?;

    let view_mat = DMat4::look_to_rh(
        DVec3::new(0.0, 0.0, 5.0),
        DVec3::new(0.0, 0.0, -1.0),
        DVec3::new(0.0, 1.0, 0.0),
    );
    let aspect_ratio = (width * FULL_BLOCK_WIDTH) as f64 / (height * FULL_BLOCK_HEIGHT) as f64;
    let proj_mat = DMat4::perspective_rh(45.0f64.to_radians(), aspect_ratio, 0.01, 1000.0);

    let framebuffer = Framebuffer::new(width, height, color::Rgb(98, 9, 92));
    let mut renderer = Renderer::new(framebuffer);
    let start_time = Instant::now();
    'game_loop: loop {
        for c in stdin.by_ref().bytes() {
            if c? == b'q' {
                break 'game_loop Ok(());
            }
        }

        renderer.clear();

        let time = start_time.elapsed().as_secs_f64();
        let model_mat = DMat4::from_axis_angle(DVec3::new(1.0, 1.0, 1.0).normalize(), time);
        let transformed_vertices =
            VERTICES.map(|pos| (proj_mat * view_mat * model_mat).project_point3(pos));

        let normals = [
            DVec3::new(0.0, 0.0, 1.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(0.0, -1.0, 0.0),
            DVec3::new(-1.0, 0.0, 0.0),
            DVec3::new(0.0, 0.0, -1.0),
        ];

        for (i, &(vert_a, vert_b, vert_c)) in TRIANGLES.iter().enumerate() {
            let normal = normals[i / 2];
            let r = ((normal.x * 0.5 + 0.5) * 255.0).round() as u8;
            let g = ((normal.y * 0.5 + 0.5) * 255.0).round() as u8;
            let b = ((normal.z * 0.5 + 0.5) * 255.0).round() as u8;

            renderer.draw_triangle(
                transformed_vertices[vert_a].xy(),
                transformed_vertices[vert_b].xy(),
                transformed_vertices[vert_c].xy(),
                Pixel::new(
                    char::from_digit(i as u32 / 2 + 1, 10).unwrap(),
                    color::Rgb(r, g, b),
                ),
            );
        }

        for (a, b) in LINES {
            renderer.draw_line(
                transformed_vertices[a].xy(),
                transformed_vertices[b].xy(),
                Pixel::new('*', color::Rgb(255, 255, 255)),
            );
        }

        write!(stdout, "{}", cursor::Goto(1, 1))?;
        renderer.present(&mut stdout)?;
        stdout.flush()?;
    }
}
