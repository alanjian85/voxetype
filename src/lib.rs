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
pub use framebuffer::{Framebuffer, Pixel};
pub use render::{
    Renderer,
    models::{LINES, TRIANGLES, VERTICES},
};
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

    let framebuffer = Framebuffer::new(width, height, color::Rgb(98, 9, 92));
    let mut renderer = Renderer::new(framebuffer);
    renderer.set_vertex_buf(VERTICES.to_vec());

    let mut timer = Timer::new();
    let mut camera = Camera::new(50.0, 5.0);
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
        renderer.set_transform_mat(proj_mat * view_mat * model_mat);

        renderer.clear();
        renderer.draw_triangles_index(&TRIANGLES, Pixel::new('0', color::Rgb(255, 255, 255)));
        renderer.draw_lines_index(&LINES, Pixel::new('*', color::Rgb(255, 255, 255)));
        renderer.present(&mut stdout)?;
    }

    write!(stdout, "{}", cursor::Show)?;
    Ok(())
}
