
use super::{ Noise, SimplexNoise };

const DEFAULT_OCTAVES: u8 = 4;
const DEFAULT_ROUGHNESS: f32 = 0.5;
const DEFAULT_SCALE: f32 = 2.5e-1;
const DEFAULT_RANGE: (f32, f32) = (-1., 1.);

pub struct OctavedNoise {
    noise: Box<Noise>,
    octaves: u8,
    roughness: f32,
    scale: f32,
    range: (f32, f32)
}

impl OctavedNoise {

    pub fn new(octaves: u8, roughness: f32, scale: f32, range: (f32, f32), noise: Box<Noise>) -> Self {
        Self {
            noise: noise,
            octaves: octaves,
            roughness: roughness,
            scale: scale,
            range: range
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

    pub fn set_range(&mut self, new_range: (f32, f32)) {
        self.range = new_range;
    }
}

/*
    Octave calculation based on code by
    matheus23 @ http://www.java-gaming.org/index.php?topic=31637.0
*/

impl Noise for OctavedNoise {
    fn get_noise(&self, p: (f32, f32)) -> f32 {
        let mut sum: f32 = 0.;
        let mut freq = self.scale;
        let mut weight: f32 = 1.;
        let mut weight_sum: f32 = 0.;

        for _oct in 0..self.octaves {
            sum += self.noise.get_noise((p.0 * freq, p.1 * freq)) * weight;
            weight_sum += weight;
            freq *= 2.;
            weight *= self.roughness;
        }
        let sub_range = self.noise.get_range();
        let normalized =  (-sub_range.0 + (sum / weight_sum)) / (sub_range.1 - sub_range.0);
        debug_assert!(normalized >= 0. && normalized <= 1.);
        self.range.0 + (self.range.1 - self.range.0) * normalized
    }

    fn get_range(&self) -> (f32, f32) {
        self.range
    }
}

impl Default for OctavedNoise {
    fn default() -> Self {
        Self {
            noise: Box::new(SimplexNoise::default()),
            octaves: DEFAULT_OCTAVES,
            roughness: DEFAULT_ROUGHNESS,
            scale: DEFAULT_SCALE,
            range: DEFAULT_RANGE
        }
    }
}
