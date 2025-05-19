use std::{
    error::Error,
    io::{self, Read, Write},
    thread,
    time::Duration,
};
use termion::{color, cursor, raw::IntoRawMode};

pub mod framebuffer;

pub use framebuffer::{Framebuffer, Pixel};

pub fn run(width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout()
        .lock()
        .into_raw_mode()
        .expect("unable to switch stdout to raw mode");
    let mut stdin = termion::async_stdin();
    write!(stdout, "{}", cursor::Hide)?;

    let mut framebuffer = Framebuffer::new(width, height);
    let mut time = 0.0;
    'game_loop: loop {
        for c in stdin.by_ref().bytes() {
            if c? == b'q' {
                break 'game_loop Ok(());
            }
        }

        for y in 0..height {
            for x in 0..width {
                let u = x as f64 / (width - 1) as f64;
                let v = y as f64 / (height - 1) as f64;

                let r = (time + u + 0.0).cos() * 0.5 + 0.5;
                let g = (time + v + 2.0).cos() * 0.5 + 0.5;
                let b = (time + u + 4.0).cos() * 0.5 + 0.5;

                let r = (r * 256.0).clamp(0.0, 255.0) as u8;
                let g = (g * 256.0).clamp(0.0, 255.0) as u8;
                let b = (b * 256.0).clamp(0.0, 255.0) as u8;

                framebuffer.write(x, y, Pixel('█', color::Rgb(r, g, b)));
            }
        }

        framebuffer.present(&mut stdout)?;
        stdout.flush()?;

        time += 0.016;
        thread::sleep(Duration::from_millis(16));
    }
}
