use glam::{DMat4, DVec3};

pub struct Camera {
    speed: f64,
    radius: f64,
    theta: f64,
}

impl Camera {
    pub fn new(speed: f64, radius: f64) -> Self {
        Self {
            speed,
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

    pub fn view_mat(&self) -> DMat4 {
        let theta = self.theta.to_radians() * 5.0;
        DMat4::look_at_rh(
            DVec3::new(theta.sin(), 0.0, theta.cos()) * self.radius,
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        )
    }

    pub fn handle_key(&mut self, c: u8, delta_time: f64) {
        let diff = delta_time * self.speed;
        match c {
            b'w' => self.radius -= diff,
            b'a' => self.theta += diff,
            b's' => self.radius += diff,
            b'd' => self.theta -= diff,
            _ => (),
        }
    }
}
