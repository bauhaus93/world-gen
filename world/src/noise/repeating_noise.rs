use super::Noise;
use core::Point2f;

pub struct RepeatingNoise {
    noise: Box<dyn Noise>,
    size: Point2f,
}

impl RepeatingNoise {
    pub fn wrap(wrapped_noise: Box<dyn Noise>, size: Point2f) -> Self {
        Self {
            noise: wrapped_noise,
            size: size,
        }
    }
}

impl Noise for RepeatingNoise {
    fn get_noise(&self, point: Point2f) -> f32 {
        self.noise.get_noise(point)
    }


    fn get_range(&self) -> [f32; 2] {
        self.noise.get_range()
    }
}

fn absolute_to_repeating_pos(absolute_pos: Point2f, size: Point2f) -> Point2f {
    Point2f::new(
        absolute_pos[0].rem_euclid(size[0]),
        absolute_pos[1].rem_euclid(size[1]),
    )
}
