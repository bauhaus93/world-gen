use rand;
use rand::prelude::StdRng;
use rand::Rng;
use std::f32;

use super::Noise;
use crate::chunk::CHUNK_SIZE;
use core::{Point2f, Point2i, Seed};

pub struct WorleyNoise {
    seed: Seed,
    grid_size: i32,
}

impl Noise for WorleyNoise {
    fn get_noise(&self, p: Point2f) -> f32 {
        self.get_min_distance(p) / self.get_single_max()
    }

    fn get_range(&self) -> [f32; 2] {
        [0., 1.]
    }

    fn get_cycle(&self) -> Point2f {
        Point2f::from_scalar(f32::INFINITY)
    }
}

impl WorleyNoise {
    pub fn from_seed(seed: Seed) -> Self {
        Self {
            seed: seed,
            grid_size: CHUNK_SIZE,
        }
    }
    fn get_min_distance(&self, p: Point2f) -> f32 {
        const GRID_OFFSETS: [[i32; 2]; 9] = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
            [0, 0],
            [1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
        ];
        let ref_grid = world_to_grid_pos(p, self.grid_size);

        let mut min_distance = f32::INFINITY;
        for off in GRID_OFFSETS.iter() {
            let grid_pos = ref_grid + Point2i::new(off[0], off[1]);
            let grid_point = self.get_point_for_grid(grid_pos);
            let distance = (grid_point - p).length();
            if distance < min_distance {
                min_distance = distance;
            }
        }
        min_distance
    }

    fn get_single_max(&self) -> f32 {
        self.grid_size as f32 * f32::sqrt(2.)
    }

    fn get_point_for_grid(&self, grid_pos: Point2i) -> Point2f {
        let mut rng: StdRng = self.seed.mix_with_point(grid_pos).into();
        Point2f::new(
            (grid_pos[0] * self.grid_size) as f32
                + rng.gen_range(0.0..self.grid_size as f32 - 1e-3),
            (grid_pos[1] * self.grid_size) as f32
                + rng.gen_range(0.0..self.grid_size as f32 - 1e-3),
        )
    }
}

fn world_to_grid_pos(world_pos: Point2f, grid_size: i32) -> Point2i {
    Point2i::from(world_pos / (grid_size as f32))
}
