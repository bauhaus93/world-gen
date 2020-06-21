use byteorder::{LittleEndian, WriteBytesExt};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{f32, io};

use super::Architect;
use crate::noise::{Noise, NoiseBuilder};
use crate::{Terrain, TerrainSet, TerrainType};
use core::{traits::Saveable, Point2f, Point2i, Seed};

pub struct NoisedArchitect {
    height_noise: Box<dyn Noise>,
    mountain_noise: Box<dyn Noise>,
    terrain_set: TerrainSet,
}

impl Architect for NoisedArchitect {
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

impl Saveable for NoisedArchitect {
    fn save(&self, writer: &mut impl io::Write) -> Result<(), io::Error> {
        if self.height_noise.is_infinite() {
            unimplemented!();
        } else {
            let cycle = Point2i::from(self.height_noise.get_cycle());
            writer.write_i32::<LittleEndian>(cycle[0])?;
            writer.write_i32::<LittleEndian>(cycle[1])?;
            for y in 0..cycle[1] {
                for x in 0..cycle[0] {
                    writer.write_f32::<LittleEndian>(self.get_height(Point2f::new(x as f32, y as f32)))?;
                    if (y * cycle[0] + x) % 1000000 == 0 {
                        info!("Writing noise...{:02}%", 100. * (y * cycle[0] + x) as f32 / (cycle[0] * cycle[1]) as f32)
                    }
                }
            }
        }
        Ok(())
    }
}

impl NoisedArchitect {
    pub fn new_finite(seed: Seed, size: Point2f, terrain_set: &TerrainSet) -> Self {
        let mut local_rng: SmallRng = seed.into();

        let height_noise = NoiseBuilder::new()
            .seed(Seed::from_rng(&mut local_rng))
            .octaves(6)
            .scale(1e-3)
            .roughness(0.5)
            .range([0., 100.])
            .repeat(size)
            .finish();

        let mountain_noise = NoiseBuilder::new()
            .seed(Seed::from_rng(&mut local_rng))
            .octaves(4)
            .scale(1e-4)
            .roughness(2.)
            .range([-1., 1.])
            .repeat(size)
            .finish();

        Self {
            height_noise: height_noise,
            mountain_noise: mountain_noise,
            terrain_set: terrain_set.clone(),
        }
    }

    pub fn new_infinite(seed: Seed, terrain_set: &TerrainSet) -> Self {
        Self::new_finite(seed, Point2f::from_scalar(f32::INFINITY), terrain_set)
    }

    fn get_mountain_factor(&self, absolute_pos: Point2f) -> f32 {
        match self.mountain_noise.get_noise(absolute_pos) {
            val if val > 0. => 1. + (10. * val.powf(2.)),
            _ => 1.,
        }
    }
}
