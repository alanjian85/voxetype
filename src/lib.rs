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

        let vertices = [(0.0, 0.57), (-0.5, -0.29), (0.5, -0.29)].map(|vertex| {
            let (x, y) = rotate(vertex, time);
            let x = x / (width as f64 / height as f64) * 2.0;
            (x, y)
        });

        let lines = [
            (vertices[0], vertices[1]),
            (vertices[1], vertices[2]),
            (vertices[2], vertices[0]),
        ];
        for (a, b) in lines {
            let c = select_char(a, b);
            renderer.draw_line(a, b, Pixel(c, color::Rgb(255, 255, 255)));
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

fn select_char(a: (f64, f64), b: (f64, f64)) -> char {
    let theta = f64::consts::PI / 8.0;
    match (b.1 - a.1).atan2(b.0 - a.0) / theta {
        1.0..=3.0 | -7.0..=-5.0 => '/',
        3.0..=5.0 | -5.0..=-3.0 => '‖',
        5.0..=7.0 | -3.0..=-1.0 => '\\',
        _ => '=',
    }
}
