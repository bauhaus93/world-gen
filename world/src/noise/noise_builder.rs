use rand::FromEntropy;
use std::ops::Fn;

use super::{
    ModifierType, Noise, NoiseModifier, OctavedNoise, RepeatingNoise, SimplexNoise, Threshold,
    ThresholdNoise,MergeType, FactoredNoise
};
use core::{Point2f, Seed};

pub struct NoiseBuilder {
    seed: Option<Seed>,
    octaves: Option<u8>,
    scale: Option<f32>,
    roughness: Option<f32>,
    range: Option<[f32; 2]>,
    modifier: Option<ModifierType>,
    threshold: Option<Threshold>,
    factor_merge_type: Option<MergeType>,
    factors: Vec<Box<dyn Noise>>,
}

impl NoiseBuilder {
    pub fn new() -> Self {
        Self {
            seed: None,
            octaves: None,
            scale: None,
            roughness: None,
            range: None,
            modifier: None,
            threshold: None,
            factor_merge_type: None,
            factors: Vec::new(),
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

    pub fn modifier(mut self, modifier: ModifierType) -> Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn below(mut self, max: f32) -> Self {
        self.threshold = Some(Threshold::Below(max));
        self
    }

    pub fn above(mut self, min: f32) -> Self {
        self.threshold = Some(Threshold::Above(min));
        self
    }

    pub fn factor_merge_type(mut self, merge_type: MergeType) -> Self {
        self.factor_merge_type = Some(merge_type);
        self
    }

    pub fn add_factor(mut self, factor: Box<dyn Noise>) -> Self {
        self.factors.push(factor);
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

    fn handle_modifier(&self, noise: Box<dyn Noise>) -> Box<dyn Noise> {
        match &self.modifier {
            Some(m) => Box::new(NoiseModifier::wrap_around(noise, *m)),
            None => noise,
        }
    }

    fn handle_threshold(&self, noise: Box<dyn Noise>) -> Box<dyn Noise> {
        match &self.threshold {
            Some(t) => Box::new(ThresholdNoise::wrap_around(noise, *t)),
            None => noise,
        }
    }

    fn handle_factors(self, noise: Box<dyn Noise>) -> Box<dyn Noise> {
        if self.factors.len() > 0 {
            let mut factored_noise = FactoredNoise::new(noise, self.factor_merge_type.unwrap_or(MergeType::SUM));
            self.factors.into_iter().for_each(|n| factored_noise.add_factor(n));
            Box::new(factored_noise)
        } else  {
            noise
        }
    }

    pub fn finish(self) -> Box<dyn Noise> {
        let n =
        self.handle_threshold(
            self.handle_modifier(self.handle_octaved_noise(self.handle_base_noise())),
        );
        self.handle_factors(n)
    }
}
