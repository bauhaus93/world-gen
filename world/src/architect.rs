use std::path::Path;

use crate::chunk::chunk_size::get_world_pos;
use crate::height_map::HeightMap;
use crate::noise::Noise;
use crate::{Terrain, TerrainSet, TerrainType};
use core::{FileError, Point2f, Point2i};

pub struct Architect {
    source: Source,
    terrain_set: TerrainSet,
}

enum Source {
    Memory(HeightMap),
    Noise(Box<dyn Noise>),
}

impl Architect {
    pub fn from_file(filepath: &Path, terrain_set: &TerrainSet) -> Result<Self, FileError> {
        Ok(Self {
            source: Source::Memory(HeightMap::from_file(filepath)?),
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
            Source::Memory(data) => data.get(absolute_pos.into()),
            Source::Noise(noise) => noise.get_noise(absolute_pos),
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

    pub fn get_terrain(&self, _absolute_pos: Point2f) -> &Terrain {
        self.terrain_set
            .get(&TerrainType::Grass)
            .expect("Must have grass")
    }
}
