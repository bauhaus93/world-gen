use rand::{rngs::SmallRng, Rng, SeedableRng};

use super::Architect;
use crate::noise::{Noise, NoiseBuilder};
use crate::{Terrain, TerrainSet, TerrainType};
use core::Point2f;

pub struct SimpleArchitect {
    height_noise: Box<dyn Noise>,
    mountain_noise: Box<dyn Noise>,
    terrain_set: TerrainSet,
}

impl Architect for SimpleArchitect {
    fn get_height(&self, absolute_pos: Point2f) -> f32 {
        let raw_height = self.height_noise.get_noise(absolute_pos);
        let mountain_factor = self.get_mountain_factor(absolute_pos);
        mountain_factor * raw_height
    }

    fn get_terrain(&self, absolute_pos: Point2f) -> &Terrain {
        self.terrain_set
            .get(&TerrainType::Grass)
            .expect("Must have grass")
    }
}

impl SimpleArchitect {
    pub fn from_rng<R: Rng + ?Sized>(
        rng: &mut R,
        world_size: Point2f,
        terrain_set: &TerrainSet,
    ) -> Self {
        let mut local_rng = SmallRng::from_rng(rng).unwrap();

        let height_noise = NoiseBuilder::new()
            .seed_by_rng(&mut local_rng)
            .octaves(6)
            .scale(1e-3)
            .roughness(0.5)
            .range([0., 100.])
            .repeat(world_size)
            .finish();

        let mountain_noise = NoiseBuilder::new()
            .seed_by_rng(&mut local_rng)
            .octaves(4)
            .scale(1e-4)
            .roughness(2.)
            .range([-1., 1.])
            .repeat(world_size)
            .finish();

        Self {
            height_noise: height_noise,
            mountain_noise: mountain_noise,
            terrain_set: terrain_set.clone(),
        }
    }

    fn get_mountain_factor(&self, absolute_pos: Point2f) -> f32 {
        match self.mountain_noise.get_noise(absolute_pos) {
            val if val > 0. => 1. + (10. * val.powf(2.)),
            _ => 1.,
        }
    }
}
