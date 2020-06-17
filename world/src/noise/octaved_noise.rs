
use rand::Rng;

use core::Point2f;
use super::{ Noise, SimplexNoise };

const DEFAULT_OCTAVES: u8 = 4;
const DEFAULT_ROUGHNESS: f32 = 0.8;
const DEFAULT_SCALE: f32 = 1e-2;
const DEFAULT_RANGE: [f32; 2]= [-1., 1.];

pub struct OctavedNoise {
    noise: Box<dyn Noise>,
    octaves: u8,
    roughness: f32,
    scale: f32,
    range: [f32; 2]
}

impl OctavedNoise {

    #[allow(dead_code)]
    pub fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            noise: Box::new(SimplexNoise::from_rng(rng)),
            octaves: DEFAULT_OCTAVES,
            roughness: DEFAULT_ROUGHNESS,
            scale: DEFAULT_SCALE,
            range: DEFAULT_RANGE
        }
    }

    pub fn set_octaves(&mut self, octave_count: u8) {
        self.octaves = octave_count;
    }

    pub fn set_roughness(&mut self, roughness: f32) {
        self.roughness = roughness;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_range(&mut self, new_range: [f32; 2]) {
        self.range = new_range;
    }
}

/*
    Octave calculation based on code by
    matheus23 @ http://www.java-gaming.org/index.php?topic=31637[0]
*/

impl Noise for OctavedNoise {
    fn get_noise(&self, p: Point2f) -> f32 {
        let mut sum: f32 = 0.;
        let mut freq = self.scale;
        let mut weight: f32 = 1.;
        let mut weight_sum: f32 = 0.;

        for _oct in 0..self.octaves {
            sum += self.noise.get_noise(p * freq) * weight;
            weight_sum += weight;
            freq *= 2.;
            weight *= self.roughness;
        }
        let sub_range = self.noise.get_range();
        let normalized =  (-sub_range[0] + (sum / weight_sum)) / (sub_range[1] - sub_range[0]);
        debug_assert!(normalized >= 0. && normalized <= 1.);
        let value = self.range[0] + (self.range[1] - self.range[0]) * normalized;
        value
    }

    fn get_range(&self) -> [f32; 2] {
        self.range
    }
}
