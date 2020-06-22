use rand::FromEntropy;
use std::ops::Fn;

use super::{Noise, OctavedNoise, RepeatingNoise, SimplexNoise};
use core::{Point2f, Seed};

pub struct NoiseBuilder {
    seed: Option<Seed>,
    octaves: Option<u8>,
    scale: Option<f32>,
    roughness: Option<f32>,
    range: Option<[f32; 2]>,
    modifier: Option<([f32; 2], Box<dyn Fn(f32) -> f32 + Send + Sync>)>
}

impl NoiseBuilder {
    pub fn new() -> Self {
        Self {
            seed: None,
            octaves: None,
            scale: None,
            roughness: None,
            range: None,
            modifier: None
        }
    }

    pub fn seed(mut self, seed: Seed) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn octaves(mut self, octaves: u8) -> Self {
        self.octaves = Some(octaves);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn roughness(mut self, roughness: f32) -> Self {
        self.roughness = Some(roughness);
        self
    }

    pub fn range(mut self, range: [f32; 2]) -> Self {
        self.range = Some(range);
        self
    }

    pub fn modifier(mut self, range: [f32; 2], modifier: Box<dyn Fn(f32) -> f32 + Send + Sync>) -> Self {
        self.modifier = Some((range, modifier));
        self
    }

    fn handle_base_noise(&self) -> Box<dyn Noise> {
        Box::new(SimplexNoise::from_seed(
            self.seed.unwrap_or(Seed::from_entropy()),
        ))
    }

    fn handle_octaved_noise(&self, noise: Box<dyn Noise>) -> Box<dyn Noise> {
        match self.octaves {
            Some(oct) => {
                let mut oct_noise = OctavedNoise::wrap(noise);
                oct_noise.set_octaves(oct);
                oct_noise.set_scale(self.scale.unwrap_or(1e-2));
                oct_noise.set_roughness(self.roughness.unwrap_or(0.8));
                oct_noise.set_range(self.range.unwrap_or([-1., 1.]));
                Box::new(oct_noise)
            }
            None => match self.range {
                Some(range) => {
                    let mut oct_noise = OctavedNoise::wrap(noise);
                    oct_noise.set_octaves(1);
                    oct_noise.set_scale(1.);
                    oct_noise.set_range(range);
                    Box::new(oct_noise)
                }
                None => noise,
            },
        }
    }

    pub fn finish(self) -> Box<dyn Noise> {
        self.handle_octaved_noise(self.handle_base_noise())
    }
}
