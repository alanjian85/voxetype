use std::{
    error::Error,
    io::{self, Write},
    thread,
    time::Duration,
};
use termion::{color, cursor};

fn main() {
    let (width, height) = termion::terminal_size().expect("unable to fetch the terminal size");

    if let Err(e) = run(width as usize, height as usize) {
        eprintln!("Application error: {e}");
    }
}

fn run(width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    print!("{}", cursor::Hide);

    let mut time = 0.0;
    loop {
        print!("{}", cursor::Goto(1, 1));
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

                print!("{}█", color::Fg(color::Rgb(r, g, b)));
            }

            if y != height - 1 {
                println!();
            }
        }
        io::stdout().flush()?;

        time += 0.016;
        thread::sleep(Duration::from_millis(16));
    }
}
