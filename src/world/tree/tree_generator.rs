use std::convert::TryFrom;

use rand;
use rand::{ Rng, SeedableRng };
use rand::rngs::SmallRng;
use glm::Vector3;

use crate::utility::Float;
use crate::graphics::mesh::{ Buffer, Mesh };
use crate::world::chunk::ChunkError;
use super::branch::Branch;


pub struct TreeGenerator {
    rng: SmallRng,
    part_range: [u32; 2],
    variation: Float 
}

impl TreeGenerator {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            rng: SmallRng::from_rng(rng).unwrap(),
            part_range: [10, 20],
            variation: 0.1
        }
    }

    pub fn set_part_range(&mut self, new_range: [u32; 2]) {
        self.part_range = new_range;
    }

    pub fn set_variation(&mut self, new_variation: Float) {
        self.variation = new_variation;
    }

    pub fn build_tree(&mut self, ring_vertex_count: u32) -> Result<Mesh, ChunkError> {
        let part_count = self.rng.gen_range(self.part_range[0], self.part_range[1]);
        let root_branch = Branch::new(part_count, self.variation, Vector3::new(0., 0., 1.0), &mut self.rng);
        let buffer = root_branch.build_buffer(ring_vertex_count);
        let mesh = Mesh::try_from(buffer)?;
        Ok(mesh)
    }
}
