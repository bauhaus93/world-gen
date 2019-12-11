use crate::traits::Updatable;
use crate::UpdateError;

pub struct Timer {
    accumulator: u32,
    time_threshold: u32
}

impl Timer {
    pub fn new(time_threshold: u32) -> Self {
        Self {
            accumulator: 0,
            time_threshold: time_threshold
        }
    }

    pub fn fires(&mut self) -> bool {
        if self.accumulator >= self.time_threshold {
            self.accumulator = 0;
            true
        } else { 
            false
        }
    }
}

impl Updatable for Timer {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        self.accumulator += time_passed;
        Ok(())
    }
}
