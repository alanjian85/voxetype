use std::time::Instant;

pub struct Timer {
    start_time: Instant,
    last_time: Instant,
    delta_time: f64,
}

impl Timer {
    pub fn new() -> Self {
        let start_time = Instant::now();
        Self {
            start_time,
            last_time: start_time,
            delta_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let curr_time = Instant::now();
        self.delta_time = (curr_time - self.last_time).as_secs_f64();
        self.last_time = curr_time;
    }

    pub fn time(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
