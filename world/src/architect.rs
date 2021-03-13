use rand::rngs::StdRng;
use rand::Rng;
use std::collections::BTreeSet;

use crate::chunk::{get_world_pos, CHUNK_SIZE};
use crate::height_map::HeightMap;
use crate::noise::presets::{get_default_noise, get_default_tree_noise};
use crate::noise::Noise;
use core::{Point2f, Point2i, Point3f, Seed};

pub struct Architect {
    height_noise: Box<dyn Noise>,
    tree_noise: Box<dyn Noise>,
}

impl Architect {
    pub fn from_seed(seed: Seed) -> Self {
        Self {
            height_noise: get_default_noise(seed),
            tree_noise: get_default_tree_noise(seed),
        }
    }

    pub fn get_height(&self, absolute_pos: Point2f) -> f32 {
        self.height_noise.get_noise(absolute_pos)
    }

    pub fn create_heightmap(&self, chunk_pos: Point2i) -> HeightMap {
        HeightMap::from_noise(
            get_world_pos(chunk_pos, Point2f::new(0., 0.)),
            CHUNK_SIZE,
            self.height_noise.as_ref(),
        )
    }

    pub fn get_trees(&self, chunk_pos: Point2i) -> Vec<Point3f> {
        let n = self.tree_noise.get_noise(chunk_pos.into());
        if n > 0. {
            let tree_count = (40. * n).round() as usize;
            let mut rng: StdRng = Seed::from_entropy().into();
            let mut trees = BTreeSet::new();
            for _i in 0..tree_count {
                let offset =
                    Point2i::new(rng.gen_range(0..CHUNK_SIZE), rng.gen_range(0..CHUNK_SIZE));
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
