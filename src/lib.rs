use glam::{DMat4, DVec2, DVec3, Vec3Swizzles};
use std::{
    error::Error,
    f64,
    io::{self, Read, Write},
    thread,
    time::{Duration, Instant},
};
use termion::{color, cursor, raw::IntoRawMode};

pub mod framebuffer;
pub mod render;

pub use framebuffer::{Framebuffer, Pixel};
pub use render::Renderer;

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
        DVec3::new(0.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -1.0),
        DVec3::new(0.0, 1.0, 0.0),
    );
    let aspect_ratio = (width * FULL_BLOCK_WIDTH) as f64 / (height * FULL_BLOCK_HEIGHT) as f64;
    let proj_mat = DMat4::perspective_rh(45.0f64.to_radians(), aspect_ratio, 0.01, 1000.0);

    let framebuffer = Framebuffer::new(width, height);
    let mut renderer = Renderer::new(framebuffer);
    let start_time = Instant::now();
    'game_loop: loop {
        for c in stdin.by_ref().bytes() {
            if c? == b'q' {
                break 'game_loop Ok(());
            }
        }

        renderer.clear();

        let model_mat = DMat4::from_translation(DVec3::new(0.0, 0.0, -5.0))
            * DMat4::from_rotation_y(start_time.elapsed().as_secs_f64() * 2.0);
        let vertices_pos = [
            DVec3::new(0.0, 0.87, 0.0),
            DVec3::new(-1.0, -0.87, 0.0),
            DVec3::new(1.0, -0.87, 0.0),
        ]
        .map(|pos| (proj_mat * view_mat * model_mat).project_point3(pos));

        let lines = [
            (vertices_pos[0], vertices_pos[1]),
            (vertices_pos[1], vertices_pos[2]),
            (vertices_pos[2], vertices_pos[0]),
        ];
        for (a, b) in lines {
            renderer.draw_line(a.xy(), b.xy(), Pixel::new('*', color::Rgb(255, 255, 255)));
        }

        write!(stdout, "{}", cursor::Goto(1, 1))?;
        renderer.present(&mut stdout)?;
        stdout.flush()?;
    }
}

fn rotate(pos: DVec2, theta: f64) -> DVec2 {
    let (sin, cos) = theta.sin_cos();
    DVec2::new(cos * pos.x - sin * pos.y, sin * pos.x + cos * pos.y)
}
