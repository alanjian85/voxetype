use glam::{DMat4, DVec3};
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

    let aspect_ratio = (width * FULL_BLOCK_WIDTH) as f64 / (height * FULL_BLOCK_HEIGHT) as f64;
    let proj_mat = DMat4::perspective_rh(45.0f64.to_radians(), aspect_ratio, 0.01, 1000.0);

    let framebuffer = Framebuffer::new(width, height, color::Rgb(98, 9, 92));
    let mut renderer = Renderer::new(framebuffer);
    renderer.set_vertex_buf(VERTICES.to_vec());

    let mut cam_pos = DVec3::new(0.0, 0.0, 5.0);
    let start_time = Instant::now();
    'game_loop: loop {
        for c in stdin.by_ref().bytes() {
            match c? {
                b'q' => break 'game_loop,
                b'w' => cam_pos.z -= 0.1,
                b'a' => cam_pos.x -= 0.1,
                b's' => cam_pos.z += 0.1,
                b'd' => cam_pos.x += 0.1,
                _ => (),
            }
        }

        renderer.clear();

        let time = start_time.elapsed().as_secs_f64();
        let model_mat = DMat4::from_axis_angle(DVec3::new(1.0, 1.0, 1.0).normalize(), time);
        let view_mat = DMat4::look_to_rh(
            cam_pos,
            DVec3::new(0.0, 0.0, -1.0),
            DVec3::new(0.0, 1.0, 0.0),
        );
        renderer.set_transform_mat(proj_mat * view_mat * model_mat);

        renderer.draw_triangles_index(&TRIANGLES, Pixel::new('0', color::Rgb(255, 255, 255)));
        renderer.draw_lines_index(&LINES, Pixel::new('*', color::Rgb(255, 255, 255)));

        write!(stdout, "{}", cursor::Goto(1, 1))?;
        renderer.present(&mut stdout)?;
        stdout.flush()?;
    }

    write!(stdout, "{}", cursor::Show)?;
    Ok(())
}
