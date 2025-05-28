use glam::DVec2;
use std::{fs, io, path::Path};
use termion::color;

pub struct Texture {
    width: usize,
    height: usize,
    texels: Vec<(char, color::Rgb)>,
}

impl Texture {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Texture> {
        let bytes = fs::read(path)?;
        let width = 16;
        let height = 16;
        let mut texels = Vec::new();

        for texel in bytes.chunks_exact(4) {
            let glyph = texel[0] as char;
            let r = texel[1];
            let g = texel[2];
            let b = texel[3];
            texels.push((glyph, color::Rgb(r, g, b)));
        }

        Ok(Self {
            width,
            height,
            texels,
        })
    }

    pub fn read(&self, x: usize, y: usize) -> (char, color::Rgb) {
        self.texels[y * self.width + x]
    }

    pub fn sample(&self, uv: DVec2) -> (char, color::Rgb) {
        let x = (uv.x * (self.width - 1) as f64).round() as usize;
        let y = ((1.0 - uv.y) * (self.height - 1) as f64).round() as usize;
        self.read(x, y)
    }
}
