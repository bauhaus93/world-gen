use rand::{rngs::SmallRng, SeedableRng};
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::{f32, io};

use crate::chunk::chunk_size::get_world_pos;
use crate::chunk::height_map::HeightMap;
use crate::noise::{Noise, NoiseBuilder};
use crate::{Terrain, TerrainSet, TerrainType};
use byteorder::{LittleEndian, ReadBytesExt};
use core::{Point2f, Point2i, Seed};

pub struct Architect {
    source: Source,
    terrain_set: TerrainSet,
}

enum Source {
    File(BufReader<File>, Point2i),
    Noise(Box<dyn Noise>),
}

impl Architect {
    pub fn from_file(filepath: &Path, terrain_set: &TerrainSet) -> Result<Self, io::Error> {
        let mut file = File::open(filepath)?;

        let size = Point2i::new(
            file.read_i32::<LittleEndian>()?,
            file.read_i32::<LittleEndian>()?,
        );

        Ok(Self {
            source: Source::File(BufReader::new(file), size),
            terrain_set: terrain_set.clone(),
        })
    }

    pub fn from_noise(noise: Box<dyn Noise>, terrain_set: &TerrainSet) -> Self {
        Self {
            source: Source::Noise(noise),
            terrain_set: terrain_set.clone(),
        }
    }

    pub fn get_height(&self, absolute_pos: Point2f) -> f32 {
        match &self.source {
            Source::File(reader, size) => 0.,
            Source::Noise(noise) => noise.get_noise(absolute_pos)
        }
    }

    pub fn create_heightmap(
        &self,
        chunk_pos: Point2i,
        chunk_size: i32,
        resolution: i32,
    ) -> HeightMap {
        let size = chunk_size + 1;
        let mut height_map = HeightMap::new(size, resolution);
        for y in 0..size {
            for x in 0..size {
                let rel_pos = Point2i::new(x, y);
                let abs_pos = get_world_pos(chunk_pos, Point2f::from(rel_pos * resolution));
                height_map.set(rel_pos, self.get_height(abs_pos));
            }
        }
        height_map
    }

    pub fn get_terrain(&self, absolute_pos: Point2f) -> &Terrain {
        self.terrain_set
            .get(&TerrainType::Grass)
            .expect("Must have grass")
    }

    /*pub fn new_finite(seed: Seed, size: Point2f, terrain_set: &TerrainSet) -> Self {
        let mut local_rng: SmallRng = seed.into();

        let height_noise = NoiseBuilder::new()
            .seed(Seed::from_rng(&mut local_rng))
            .octaves(6)
            .scale(1e-3)
            .roughness(0.5)
            .range([0., 100.])
            .finish();

        let mountain_noise = NoiseBuilder::new()
            .seed(Seed::from_rng(&mut local_rng))
            .octaves(4)
            .scale(1e-4)
            .roughness(2.)
            .range([-1., 1.])
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

    pub fn get_height(&self, absolute_pos: Point2f) -> f32 {
        let raw_height = self.height_noise.get_noise(absolute_pos);
        let mountain_factor = self.get_mountain_factor(absolute_pos);
        mountain_factor * raw_height
    }



    fn get_mountain_factor(&self, absolute_pos: Point2f) -> f32 {
        match self.mountain_noise.get_noise(absolute_pos) {
            val if val > 0. => 1. + (10. * val.powf(2.)),
            _ => 1.,
        }
    }*/
}
