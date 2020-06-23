use std::ops::Fn;
use std::sync::Arc;

use super::Noise;
use core::Point2f;

#[derive(Clone, Copy)]
pub enum ModifierType {
    Exponent(f32),
    FactoredExponent(f32, f32),
}

pub struct NoiseModifier {
    noise: Box<dyn Noise>,
    modifier: ModifierType,
}

impl NoiseModifier {
    pub fn wrap_around(noise: Box<dyn Noise>, modifier: ModifierType) -> Self {
        Self {
            noise: noise,
            modifier: modifier,
        }
    }

    fn apply_modifier(&self, value: f32) -> f32 {
        match &self.modifier {
            ModifierType::Exponent(exp) => value.powf(*exp),
            ModifierType::FactoredExponent(fac, exp) => *fac * value.powf(*exp),
        }
    }
}

impl Noise for NoiseModifier {
    fn get_noise(&self, point: Point2f) -> f32 {
        self.apply_modifier(self.noise.get_noise(point))
    }

    fn get_range(&self) -> [f32; 2] {
        let r = self.noise.get_range();
        [self.apply_modifier(r[0]), self.apply_modifier(r[1])]
    }

    fn get_cycle(&self) -> Point2f {
        self.noise.get_cycle()
    }
}
