use std::{
    error::Error,
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

        let vertices = [(0.0, 0.57), (-0.5, -0.29), (0.5, -0.29)].map(|vertex| {
            let (x, y) = rotate(vertex, time);
            let x = x / (width as f64 / height as f64) * 2.0;

            let x = ((x * 0.5 + 0.5) * width as f64).round();
            let y = ((-y * 0.5 + 0.5) * height as f64).round();

            (x, y)
        });

        for &(x, y) in &vertices {
            if !(0.0..width as f64).contains(&x) || !(0.0..height as f64).contains(&y) {
                continue;
            }

            renderer.draw_point(
                (x as usize, y as usize),
                Pixel('.', color::Rgb(255, 255, 255)),
            );
        }

        write!(stdout, "{}", cursor::Goto(1, 1))?;
        renderer.present(&mut stdout)?;
        stdout.flush()?;

        time += 0.016;
        thread::sleep(Duration::from_millis(16));
    }
}

fn rotate((x, y): (f64, f64), theta: f64) -> (f64, f64) {
    let (sin, cos) = theta.sin_cos();
    (cos * x - sin * y, sin * x + cos * y)
}
