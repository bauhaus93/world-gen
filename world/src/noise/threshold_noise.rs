use crate::noise::Noise;
use core::Point2f;

pub struct ThresholdNoise {
    noise: Box<dyn Noise>,
    threshold: Threshold,
}

#[derive(Copy, Clone)]
pub enum Threshold {
    Below(f32),
    Above(f32),
}

impl ThresholdNoise {
    pub fn wrap_around(wrapped_noise: Box<dyn Noise>, threshold: Threshold) -> Self {
        Self {
            noise: wrapped_noise,
            threshold: threshold,
        }
    }
}

impl Noise for ThresholdNoise {
    fn get_noise(&self, point: Point2f) -> f32 {
        let n = self.noise.get_noise(point);
        match self.threshold {
            Threshold::Below(max) if n <= max => n,
            Threshold::Above(min) if n >= min => n,
            _ => 0.,
        }
    }

    fn get_range(&self) -> [f32; 2] {
        let r = self.noise.get_range();
        match self.threshold {
            Threshold::Below(max) => [f32::min(r[0], max), f32::min(r[1], max)],
            Threshold::Above(min) => [f32::max(r[0], min), f32::max(r[1], min)],
        }
    }

    fn get_cycle(&self) -> Point2f {
        self.noise.get_cycle()
    }
}
