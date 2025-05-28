use glam::{DMat4, DVec3};
use std::{
    error::Error,
    f64,
    io::{self, Read, Write},
};
use termion::{color, cursor, raw::IntoRawMode};

pub mod camera;
pub mod framebuffer;
pub mod render;
pub mod time;

pub use camera::Camera;
pub use framebuffer::Framebuffer;
pub use render::{Renderer, TRIANGLES, Texture, VERTICES, Vertex};
pub use time::Timer;

const FULL_BLOCK_WIDTH: usize = 10;
const FULL_BLOCK_HEIGHT: usize = 22;

pub fn run(width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout()
        .lock()
        .into_raw_mode()
        .expect("unable to switch stdout to raw mode");
    let mut stdin = termion::async_stdin();
    write!(stdout, "{}", cursor::Hide)?;

    let mut framebuffer = Framebuffer::new(width, height);
    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let alpha = y as f64 / (framebuffer.height() - 1) as f64;
            framebuffer.write_background(
                x,
                y,
                color::Rgb(
                    ((1.0 - alpha) * 102.0 + alpha * 41.0).round() as u8,
                    ((1.0 - alpha) * 102.0 + alpha * 0.0).round() as u8,
                    ((1.0 - alpha) * 153.0 + alpha * 94.0).round() as u8,
                ),
            );
        }
    }
    let mut renderer = Renderer::new(framebuffer);
    renderer.set_vertex_buf(VERTICES.to_vec());

    let texture = Texture::load("assets/textures/dirt.bin")?;

    let mut timer = Timer::new();
    let mut camera = Camera::new(50.0, 10.0, 5.0);
    'game_loop: loop {
        timer.update();
        for c in stdin.by_ref().bytes() {
            let c = c?;
            if c == b'q' {
                break 'game_loop;
            }
            camera.handle_key(c, timer.delta_time());
        }

        let aspect_ratio = (width * FULL_BLOCK_WIDTH) as f64 / (height * FULL_BLOCK_HEIGHT) as f64;
        let proj_mat = DMat4::perspective_rh(45.0f64.to_radians(), aspect_ratio, 0.01, 1000.0);
        let view_mat = camera.view_mat();
        let model_mat = DMat4::from_axis_angle(DVec3::new(1.0, 1.0, 1.0).normalize(), timer.time());
        let normal_mat = model_mat.inverse().transpose();

        let vert_shader = |vert: Vertex| {
            let pos = proj_mat * view_mat * model_mat * vert.pos;
            let uv = vert.uv;
            let normal = normal_mat.transform_vector3(vert.normal);
            Vertex::new(pos, uv, normal)
        };
        let frag_shader = |vert: Vertex| {
            let (glyph, color) = texture.sample(vert.uv);
            let color = DVec3::new(color.0 as f64, color.1 as f64, color.2 as f64) / 255.0;
            let color = color
                * vert
                    .normal
                    .normalize()
                    .dot(camera.pos().normalize())
                    .max(0.0);
            let r = (color.x * 255.0).round() as u8;
            let g = (color.y * 255.0).round() as u8;
            let b = (color.z * 255.0).round() as u8;
            (glyph, color::Rgb(r, g, b))
        };

        renderer.clear();
        renderer.draw_triangles(&TRIANGLES[0..36], &vert_shader, &frag_shader);
        renderer.present(&mut stdout)?;
    }

    write!(stdout, "{}", cursor::Show)?;
    Ok(())
}
