use rand::{prelude::SmallRng, FromEntropy, Rng, SeedableRng};

use core::Point2f;
use super::{Noise, OctavedNoise, RepeatingNoise, SimplexNoise};

pub struct NoiseBuilder {
    seed: Option<[u8; 16]>,
    octaves: Option<u8>,
    scale: Option<f32>,
    roughness: Option<f32>,
    range: Option<[f32; 2]>,
    repeat_cycle: Option<Point2f>,
}

impl NoiseBuilder {
    pub fn new() -> Self {
        Self {
            seed: None,
            octaves: None,
            scale: None,
            roughness: None,
            range: None,
            repeat_cycle: None,
        }
    }

    pub fn seed_by_rng<R: Rng + ?Sized>(self, rng: &mut R) -> Self {
        let mut seed = [0; 16];
        rng.fill_bytes(&mut seed);
        self.seed(seed)
    }

    pub fn seed(mut self, seed: [u8; 16]) -> Self {
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

    pub fn repeat(mut self, repeat_cycle: Point2f) -> Self {
        self.repeat_cycle = Some(repeat_cycle);
        self
    }

    fn handle_base_noise(&self) -> Box<dyn Noise> {
        let mut rng = match self.seed {
            Some(seed) => SmallRng::from_seed(seed),
            None => SmallRng::from_entropy(),
        };
        Box::new(SimplexNoise::from_rng(&mut rng))
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
            }
        }
    }

    fn handle_repeating_noise(&self, noise: Box<dyn Noise>) -> Box<dyn Noise> {
        match self.repeat_cycle {
            Some(cycle) => Box::new(RepeatingNoise::wrap(noise, cycle)),
            None => noise
        }
    }

    pub fn finish(self) -> Box<Noise> {
        self.handle_repeating_noise(self.handle_octaved_noise(self.handle_base_noise()))
    }
}
