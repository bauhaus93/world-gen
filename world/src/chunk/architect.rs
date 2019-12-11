use rand::{ Rng, rngs::SmallRng, SeedableRng };

use core::Float;
use crate::noise::{ Noise, OctavedNoise };
use crate::{ Terrain, TerrainType, TerrainSet };
use super::height_map::HeightMap;
use super::get_world_pos;

pub struct Architect {
    height_noise: OctavedNoise,
    mountain_noise: OctavedNoise,
    terrain_set: TerrainSet
}

impl Architect {
    pub fn from_rng<R: Rng + ?Sized>(rng: &mut R, terrain_set: &TerrainSet) -> Architect {
        let mut local_rng = SmallRng::from_rng(rng).unwrap();

        let mut height_noise = OctavedNoise::from_rng(&mut local_rng);
        height_noise.set_octaves(6);
        height_noise.set_scale(1e-3);
        height_noise.set_roughness(0.5);
        height_noise.set_range([0., 100.]);

        let mut mountain_noise = OctavedNoise::from_rng(&mut local_rng);
        mountain_noise.set_octaves(4);
        mountain_noise.set_scale(1e-4);
        mountain_noise.set_roughness(2.);
        mountain_noise.set_range([-1., 1.]);

        Self {
            height_noise: height_noise,
            mountain_noise: mountain_noise,
            terrain_set: terrain_set.clone()
        }
    }

    pub fn create_height_map(&self, chunk_pos: [i32; 2], chunk_size: i32, resolution: i32) -> HeightMap {
        let size = chunk_size + 1;
        let mut height_map = HeightMap::new(size, resolution);
        for y in 0..size {
            for x in 0..size {
                let abs_pos = get_world_pos(&chunk_pos, &[x, y], resolution);
                height_map.set(&[x, y], self.get_height(abs_pos));
            }
        }
        height_map
    }

    pub fn get_terrain(&self, absolute_pos: [Float; 2]) -> &Terrain {
        let mountain_val = self.mountain_noise.get_noise([absolute_pos[0] as f64, absolute_pos[1] as f64]);
        let terrain = if mountain_val > 0.5 {
            self.terrain_set.get(&TerrainType::Rock)
        } else {
            self.terrain_set.get(&TerrainType::Grass)
        };
        match terrain {
            Some(t) => t,
            None => {
                error!("Requested terrain type did not exist!");
                panic!();
            }
        }
    }

    fn get_mountain_factor(&self, absolute_pos: [Float; 2]) -> f64 {
        match self.mountain_noise.get_noise([absolute_pos[0] as f64, absolute_pos[1] as f64]) {
            val if val > 0. => 1. +  (10. * val.powf(2.)),
            _ => 1.
        }
    }

    fn get_height(&self, absolute_pos: [Float; 2]) -> f64 {
        let raw_height = self.height_noise.get_noise([absolute_pos[0] as f64, absolute_pos[1] as f64]);
        let mountain_factor = self.get_mountain_factor(absolute_pos);
        mountain_factor * raw_height
    }
}
