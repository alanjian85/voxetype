fn main() {
    let (width, height) = termion::terminal_size().expect("unable to fetch the terminal size");

    if let Err(e) = voxetype::run(width as usize, height as usize) {
        eprintln!("Application error: {e}");
    }
}
