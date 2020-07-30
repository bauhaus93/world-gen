use rand::rngs::SmallRng;
use rand::Rng;
use std::collections::BTreeSet;
use std::path::Path;

use crate::chunk::chunk_size::{get_world_pos, CHUNK_SIZE};
use crate::height_map::HeightMap;
use crate::noise::presets::get_default_tree_noise;
use crate::noise::Noise;
use crate::{Terrain, TerrainSet, TerrainType};
use core::{FileError, Point2f, Point2i, Point3f, Seed};

pub struct Architect {
    source: Source,
    tree_noise: Box<dyn Noise>,
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
            tree_noise: get_default_tree_noise(Seed::from_entropy()),
            terrain_set: terrain_set.clone(),
        })
    }

    pub fn from_noise(noise: Box<dyn Noise>, terrain_set: &TerrainSet) -> Self {
        Self {
            source: Source::Noise(noise),
            tree_noise: get_default_tree_noise(Seed::from_entropy()),
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
        resolution: f32,
    ) -> HeightMap {
        let size = chunk_size + 1;
        let mut height_map = HeightMap::new(size, resolution);
        for y in 0..size {
            for x in 0..size {
                let rel_pos = Point2i::new(x, y);
                let abs_pos = get_world_pos(chunk_pos, Point2f::from(rel_pos) * resolution);
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

    pub fn get_trees(&self, chunk_pos: Point2i) -> Vec<Point3f> {
        let n = self.tree_noise.get_noise(chunk_pos.into());
        if n > 0. {
            let tree_count = (40. * n).round() as usize;
            let mut rng: SmallRng = Seed::from_entropy().into();
            let mut trees = BTreeSet::new();
            for _i in 0..tree_count {
                let offset =
                    Point2i::new(rng.gen_range(0, CHUNK_SIZE), rng.gen_range(0, CHUNK_SIZE));
                trees.insert(offset);
            }
            trees
                .into_iter()
                .map(|offset| {
                    let abs_pos = get_world_pos(chunk_pos, offset.into());
                    let height = self.get_height(abs_pos);
                    abs_pos.extend(height)
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}
