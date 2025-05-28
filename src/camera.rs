use glam::{DMat4, DVec3};

pub struct Camera {
    move_speed: f64,
    rotation_speed: f64,
    radius: f64,
    theta: f64,
}

impl Camera {
    pub fn new(move_speed: f64, rotation_speed: f64, radius: f64) -> Self {
        Self {
            move_speed,
            rotation_speed,
            radius,
            theta: 0.0,
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    pub fn pos(&self) -> DVec3 {
        DVec3::new(self.theta.sin(), 0.0, self.theta.cos()) * self.radius
    }

    pub fn view_mat(&self) -> DMat4 {
        DMat4::look_at_rh(
            self.pos(),
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        )
    }

    pub fn handle_key(&mut self, c: u8, delta_time: f64) {
        match c {
            b'w' => self.radius -= delta_time * self.move_speed,
            b'a' => self.theta += delta_time * self.rotation_speed,
            b's' => self.radius += delta_time * self.move_speed,
            b'd' => self.theta -= delta_time * self.rotation_speed,
            _ => (),
        }
    }
}
