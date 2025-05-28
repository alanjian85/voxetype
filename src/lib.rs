use glam::{DMat4, DVec3};
use std::{
    error::Error,
    f64,
    io::{self, Read, Write},
};
use termion::{color, cursor, raw::IntoRawMode};

pub mod camera;
pub mod framebuffer;
pub mod render;
pub mod time;

pub use camera::Camera;
pub use framebuffer::Framebuffer;
pub use render::{Renderer, TRIANGLES, VERTICES, Vertex};
pub use time::Timer;

const FULL_BLOCK_WIDTH: usize = 10;
const FULL_BLOCK_HEIGHT: usize = 22;

pub fn run(width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout()
        .lock()
        .into_raw_mode()
        .expect("unable to switch stdout to raw mode");
    let mut stdin = termion::async_stdin();
    write!(stdout, "{}", cursor::Hide)?;

    let mut framebuffer = Framebuffer::new(width, height);
    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let alpha = y as f64 / (framebuffer.height() - 1) as f64;
            framebuffer.write_background(
                x,
                y,
                color::Rgb(
                    ((1.0 - alpha) * 102.0 + alpha * 41.0).round() as u8,
                    ((1.0 - alpha) * 102.0 + alpha * 0.0).round() as u8,
                    ((1.0 - alpha) * 153.0 + alpha * 94.0).round() as u8,
                ),
            );
        }
    }
    let mut renderer = Renderer::new(framebuffer);
    renderer.set_vertex_buf(VERTICES.to_vec());

    let mut timer = Timer::new();
    let mut camera = Camera::new(50.0, 10.0, 5.0);
    'game_loop: loop {
        timer.update();
        for c in stdin.by_ref().bytes() {
            let c = c?;
            if c == b'q' {
                break 'game_loop;
            }
            camera.handle_key(c, timer.delta_time());
        }

        let aspect_ratio = (width * FULL_BLOCK_WIDTH) as f64 / (height * FULL_BLOCK_HEIGHT) as f64;
        let proj_mat = DMat4::perspective_rh(45.0f64.to_radians(), aspect_ratio, 0.01, 1000.0);
        let view_mat = camera.view_mat();
        let model_mat = DMat4::from_axis_angle(DVec3::new(1.0, 1.0, 1.0).normalize(), timer.time());
        renderer.set_model_mat(model_mat);
        renderer.set_transform_mat(proj_mat * view_mat * model_mat);

        renderer.clear();
        renderer.draw_triangles(&TRIANGLES[0..36], |uv, normal| {
            let alphabet = ['#', '%', '&', '*', '@', '$', '0'];
            let palette = [
                DVec3::new(0.73, 0.52, 0.36),
                DVec3::new(0.59, 0.42, 0.29),
                DVec3::new(0.47, 0.33, 0.23),
                DVec3::new(0.35, 0.24, 0.16),
                DVec3::new(0.53, 0.53, 0.53),
                DVec3::new(0.42, 0.42, 0.42),
                DVec3::new(0.45, 0.35, 0.27),
            ];
            let texture = [
                [0, 1, 1, 2, 2, 0, 1, 1, 2, 2, 3, 2, 2, 0, 2, 0],
                [2, 1, 3, 2, 2, 1, 4, 3, 2, 0, 1, 2, 0, 1, 3, 3],
                [0, 2, 2, 3, 0, 2, 2, 2, 0, 2, 2, 2, 3, 3, 0, 2],
                [1, 5, 0, 2, 1, 3, 2, 0, 1, 1, 2, 1, 2, 0, 1, 2],
                [1, 2, 1, 0, 3, 1, 2, 2, 1, 3, 2, 5, 2, 1, 3, 2],
                [2, 3, 1, 1, 2, 1, 3, 3, 3, 2, 2, 3, 2, 2, 2, 1],
                [0, 2, 2, 2, 4, 2, 2, 0, 0, 2, 0, 0, 2, 1, 2, 1],
                [2, 2, 0, 0, 1, 1, 2, 2, 1, 3, 1, 1, 2, 2, 1, 1],
                [1, 2, 2, 1, 2, 1, 2, 3, 2, 1, 1, 2, 2, 2, 3, 2],
                [2, 1, 3, 2, 2, 3, 3, 2, 2, 2, 2, 2, 0, 0, 2, 1],
                [2, 1, 2, 0, 0, 2, 0, 1, 3, 0, 0, 3, 1, 1, 4, 2],
                [1, 2, 2, 1, 1, 0, 2, 1, 5, 1, 1, 2, 3, 1, 2, 3],
                [2, 3, 1, 2, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 0, 0],
                [2, 1, 2, 2, 6, 2, 1, 1, 2, 3, 0, 3, 2, 0, 1, 1],
                [1, 2, 3, 0, 2, 3, 2, 3, 0, 0, 2, 1, 2, 2, 1, 1],
                [1, 2, 0, 1, 1, 2, 4, 2, 1, 1, 2, 2, 1, 1, 2, 3],
            ];
            let x = (uv.x * (texture[0].len() - 1) as f64).round() as usize;
            let y = ((1.0 - uv.y) * (texture.len() - 1) as f64).round() as usize;
            let texel = texture[x][y];
            let color = palette[texel] * camera.pos().normalize().dot(normal).max(0.0);
            let r = (color.x * 255.0).round() as u8;
            let g = (color.y * 255.0).round() as u8;
            let b = (color.z * 255.0).round() as u8;
            (alphabet[texel], color::Rgb(r, g, b))
        });
        renderer.present(&mut stdout)?;
    }

    write!(stdout, "{}", cursor::Show)?;
    Ok(())
}
