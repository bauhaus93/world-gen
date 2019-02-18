use crate::utility::Float;
use crate::world::noise::{ Noise, OctavedNoise };

pub struct Architect {
    height_noise: OctavedNoise,
    hill_noise: OctavedNoise,
    mountain_noise: OctavedNoise
}


impl Architect {

    pub fn get_height(&self, absolute_pos: [Float; 2]) -> Float {
        let raw_height = self.height_noise.get_noise(absolute_pos);
        let hill_val = self.hill_noise.get_noise(absolute_pos);
        let mountain_val = self.mountain_noise.get_noise(absolute_pos);
        if mountain_val > 0. {
            raw_height * (1. +  (50. * mountain_val.powf(2.)))
        } else {
            raw_height
        }
    }

    pub fn get_ground_texture(&self, absolute_pos: [Float; 2]) -> i32 {
        let mountain_val = self.mountain_noise.get_noise(absolute_pos);
        if mountain_val > 0. {
            0
        } else {
            1
        }
    } 
}

impl Default for Architect {
    fn default() -> Self {
        let mut height_noise = OctavedNoise::default();
        height_noise.set_octaves(6);
        height_noise.set_scale(1e-3);
        height_noise.set_roughness(0.5);
        height_noise.set_range([0., 100.]);

        let mut hill_noise = OctavedNoise::default();
        let mut mountain_noise = OctavedNoise::default();
        mountain_noise.set_octaves(4);
        mountain_noise.set_scale(1e-4);
        mountain_noise.set_roughness(1.33);
        mountain_noise.set_range([-2., 1.]);

        Self {
            height_noise: height_noise,
            hill_noise: hill_noise,
            mountain_noise: mountain_noise
        }
    }
}
