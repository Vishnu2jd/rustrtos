use std::time::{Duration, Instant};
use chrono::Local;

#[allow(dead_code)]
pub struct Timer {
    start: Instant,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

#[allow(dead_code)]
pub fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[allow(dead_code)]
pub fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}
