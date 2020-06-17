use rand::{rngs::SmallRng, Rng, SeedableRng};

use super::{Architect};
use crate::noise::{Noise, OctavedNoise};
use crate::{Terrain, TerrainSet, TerrainType};
use core::Point2f;

pub struct ArchitectNoiseInfinite {
    height_noise: OctavedNoise,
    mountain_noise: OctavedNoise,
    terrain_set: TerrainSet,
}

impl Architect for ArchitectNoiseInfinite {
    fn get_height(&self, absolute_pos: Point2f) -> f32 {
        let raw_height = self.height_noise.get_noise(absolute_pos);
        let mountain_factor = self.get_mountain_factor(absolute_pos);
        mountain_factor * raw_height
    }

    fn get_terrain(&self, absolute_pos: Point2f) -> &Terrain {
        let mountain_val = self.mountain_noise.get_noise(absolute_pos);
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
}

impl ArchitectNoiseInfinite {
    pub fn from_rng<R: Rng + ?Sized>(
        rng: &mut R,
        terrain_set: &TerrainSet,
    ) -> ArchitectNoiseInfinite {
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
            terrain_set: terrain_set.clone(),
        }
    }

    fn get_mountain_factor(&self, absolute_pos: Point2f) -> f32 {
        match self
            .mountain_noise
            .get_noise(absolute_pos)
        {
            val if val > 0. => 1. + (10. * val.powf(2.)),
            _ => 1.,
        }
    }
}
