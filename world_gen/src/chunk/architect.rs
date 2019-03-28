use rand::{ Rng, rngs::SmallRng, SeedableRng };

use utility::Float;
use crate::noise::{ Noise, OctavedNoise };
use super::height_map::HeightMap;
use super::chunk_size::CHUNK_SIZE;

pub struct Architect {
    height_noise: OctavedNoise,
    hill_noise: OctavedNoise,
    mountain_noise: OctavedNoise
}


impl Architect {
    pub fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Architect {
        let mut local_rng = SmallRng::from_rng(rng).unwrap();

        let mut height_noise = OctavedNoise::from_rng(&mut local_rng);
        height_noise.set_octaves(6);
        height_noise.set_scale(1e-3);
        height_noise.set_roughness(0.5);
        height_noise.set_range([0., 100.]);

        let hill_noise = OctavedNoise::from_rng(&mut local_rng);
        let mut mountain_noise = OctavedNoise::from_rng(&mut local_rng);
        mountain_noise.set_octaves(4);
        mountain_noise.set_scale(1e-4);
        mountain_noise.set_roughness(2.);
        mountain_noise.set_range([0./*-2.*/, 1.]);
        Self {
            height_noise: height_noise,
            hill_noise: hill_noise,
            mountain_noise: mountain_noise
        }
    }

    pub fn create_height_map(&self, chunk_pos: [i32; 2], chunk_size: [i32; 2], resolution: Float) -> HeightMap {
        let size = [chunk_size[0] + 1,
                    chunk_size[1] + 1];
        let mut height_map = HeightMap::new(size);
        for y in 0..size[1] {
            for x in 0..size[0] {
                let abs_pos = [(chunk_pos[0] * CHUNK_SIZE[1]) as Float + (x as Float * resolution),
                               (chunk_pos[1] * CHUNK_SIZE[0]) as Float + (y as Float * resolution)];
                height_map.set(&[x, y], self.get_height(abs_pos));
            }
        }
        height_map
    }

    fn get_height(&self, absolute_pos: [Float; 2]) -> Float {
        let raw_height = self.height_noise.get_noise(absolute_pos);
        let _hill_val = self.hill_noise.get_noise(absolute_pos);
        let mountain_val = self.mountain_noise.get_noise(absolute_pos);
        if mountain_val > 0. {
            raw_height * (1. +  (/*30.*/ 10. * mountain_val.powf(2.)))
        } else {
            raw_height
        }
    }
    #[allow(dead_code)]
    pub fn get_ground_texture(&self, absolute_pos: [Float; 2]) -> i32 {
        let mountain_val = self.mountain_noise.get_noise(absolute_pos);
        if mountain_val > 0. {
            0
        } else {
            1
        }
    } 
}