use glam::DVec2;
use std::{
    error::Error,
    f64,
    io::{self, Read, Write},
    thread,
    time::Duration,
};
use termion::{color, cursor, raw::IntoRawMode};

pub mod framebuffer;
pub mod render;

pub use framebuffer::{Framebuffer, Pixel};
pub use render::Renderer;

pub fn run(width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout()
        .lock()
        .into_raw_mode()
        .expect("unable to switch stdout to raw mode");
    let mut stdin = termion::async_stdin();
    write!(stdout, "{}", cursor::Hide)?;

    let framebuffer = Framebuffer::new(width, height);
    let mut renderer = Renderer::new(framebuffer);
    let mut time = 0.0f64;
    'game_loop: loop {
        for c in stdin.by_ref().bytes() {
            if c? == b'q' {
                break 'game_loop Ok(());
            }
        }

        renderer.clear();

        let vertices_pos = [
            DVec2::new(0.0, 0.57),
            DVec2::new(-0.5, -0.29),
            DVec2::new(0.5, -0.29),
        ]
        .map(|pos| {
            let mut pos = rotate(pos, time);
            pos.x /= (width as f64 / height as f64) * 0.5;
            pos
        });

        let lines = [
            (vertices_pos[0], vertices_pos[1]),
            (vertices_pos[1], vertices_pos[2]),
            (vertices_pos[2], vertices_pos[0]),
        ];
        for (a, b) in lines {
            renderer.draw_line(a, b, Pixel::new('*', color::Rgb(255, 255, 255)));
        }

        write!(stdout, "{}", cursor::Goto(1, 1))?;
        renderer.present(&mut stdout)?;
        stdout.flush()?;

        time += 0.016;
        thread::sleep(Duration::from_millis(16));
    }
}

fn rotate(pos: DVec2, theta: f64) -> DVec2 {
    let (sin, cos) = theta.sin_cos();
    DVec2::new(cos * pos.x - sin * pos.y, sin * pos.x + cos * pos.y)
}
