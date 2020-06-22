use std::ops::Fn;
use std::sync::Arc;

use super::Noise;
use core::Point2f;

pub struct NoiseModifier {
    noise: Box<dyn Noise>,
    range: [f32; 2],
    modifier: Box<dyn Fn(f32) -> f32 + Send + Sync>
}

impl NoiseModifier {
    pub fn wrap_around(noise: Box<dyn Noise>, range: [f32; 2], modifier: Box<dyn Fn(f32) -> f32 + Send + Sync>) -> Self {
        Self {
            noise: noise,
            range: range,
            modifier: modifier
        }
    }
}

impl Noise for NoiseModifier {
    fn get_noise(&self, point: Point2f) -> f32 {
        (self.modifier)(self.noise.get_noise((point)))
    }

    fn get_range(&self) -> [f32; 2] {
        self.range
    }

    fn get_cycle(&self) -> Point2f {
        self.noise.get_cycle()
    }
}
