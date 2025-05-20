use glam::{DMat4, DVec3, Vec3Swizzles};
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
        DVec3::new(0.0, 0.0, 5.0),
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

        let time = start_time.elapsed().as_secs_f64();
        let model_mat = DMat4::from_axis_angle(DVec3::new(1.0, 1.0, 1.0).normalize(), time);
        let vertices_pos = [
            // front
            DVec3::new(-1.0, 1.0, 1.0),
            DVec3::new(-1.0, -1.0, 1.0),
            DVec3::new(1.0, -1.0, 1.0),
            DVec3::new(1.0, 1.0, 1.0),
            // right
            DVec3::new(1.0, 1.0, 1.0),
            DVec3::new(1.0, -1.0, 1.0),
            DVec3::new(1.0, -1.0, -1.0),
            DVec3::new(1.0, 1.0, -1.0),
            // top
            DVec3::new(-1.0, 1.0, -1.0),
            DVec3::new(-1.0, 1.0, 1.0),
            DVec3::new(1.0, 1.0, 1.0),
            DVec3::new(1.0, 1.0, -1.0),
            // bottom
            DVec3::new(-1.0, -1.0, 1.0),
            DVec3::new(-1.0, -1.0, -1.0),
            DVec3::new(1.0, -1.0, -1.0),
            DVec3::new(1.0, -1.0, 1.0),
            // left
            DVec3::new(-1.0, 1.0, -1.0),
            DVec3::new(-1.0, -1.0, -1.0),
            DVec3::new(-1.0, -1.0, 1.0),
            DVec3::new(-1.0, 1.0, 1.0),
            // back
            DVec3::new(1.0, 1.0, -1.0),
            DVec3::new(1.0, -1.0, -1.0),
            DVec3::new(-1.0, -1.0, -1.0),
            DVec3::new(-1.0, 1.0, -1.0),
        ]
        .map(|pos| (proj_mat * view_mat * model_mat).project_point3(pos));

        let lines = [
            // front
            (vertices_pos[0], vertices_pos[1]),
            (vertices_pos[1], vertices_pos[2]),
            (vertices_pos[2], vertices_pos[3]),
            (vertices_pos[3], vertices_pos[0]),
            // right
            (vertices_pos[4], vertices_pos[5]),
            (vertices_pos[5], vertices_pos[6]),
            (vertices_pos[6], vertices_pos[7]),
            (vertices_pos[7], vertices_pos[4]),
            // top
            (vertices_pos[8], vertices_pos[9]),
            (vertices_pos[9], vertices_pos[10]),
            (vertices_pos[10], vertices_pos[11]),
            (vertices_pos[11], vertices_pos[8]),
            // bottom
            (vertices_pos[12], vertices_pos[13]),
            (vertices_pos[13], vertices_pos[14]),
            (vertices_pos[14], vertices_pos[15]),
            (vertices_pos[15], vertices_pos[12]),
            // left
            (vertices_pos[16], vertices_pos[17]),
            (vertices_pos[17], vertices_pos[18]),
            (vertices_pos[18], vertices_pos[19]),
            (vertices_pos[19], vertices_pos[16]),
            // back
            (vertices_pos[20], vertices_pos[21]),
            (vertices_pos[21], vertices_pos[22]),
            (vertices_pos[22], vertices_pos[23]),
            (vertices_pos[23], vertices_pos[20]),
        ];
        for (a, b) in lines {
            renderer.draw_line(a.xy(), b.xy(), Pixel::new('*', color::Rgb(255, 255, 255)));
        }

        write!(stdout, "{}", cursor::Goto(1, 1))?;
        renderer.present(&mut stdout)?;
        stdout.flush()?;
    }
}
