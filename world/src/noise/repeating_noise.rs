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
        let rep_pos = get_repeating_pos(point, self.size);
        let pos_x = Point2f::new(self.size[0] - rep_pos[0], rep_pos[1]);
        let pos_y = Point2f::new(rep_pos[0], self.size[1] - rep_pos[1]);
        let pos_xy = Point2f::new(self.size[0] - rep_pos[0], self.size[1] - rep_pos[1]);


        (self.noise.get_noise(rep_pos) +
         self.noise.get_noise(pos_x) +
         self.noise.get_noise(pos_y) +
         self.noise.get_noise(pos_xy)) / 4.
    }

    fn get_range(&self) -> [f32; 2] {
        self.noise.get_range()
    }

    fn get_cycle(&self) -> Point2f {
        self.size
    }
}

fn get_repeating_pos(absolute_pos: Point2f, size: Point2f) -> Point2f {
    Point2f::new(
        absolute_pos[0].rem_euclid(size[0]),
        absolute_pos[1].rem_euclid(size[1]),
    )
}
