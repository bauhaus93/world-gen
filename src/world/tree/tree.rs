use std::convert::TryFrom;

use rand;
use rand::Rng;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use glm::Vector3;

use crate::utility::Float;
use crate::graphics::mesh::{ Buffer, Mesh };
use crate::world::chunk::ChunkError;
use super::branch::Branch;

pub struct Tree {
    radius: Float,
    length: Float,
    root: Branch
}

impl Tree {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_seed([0; 16]);
        Self {
            radius: rng.gen_range(1., 3.),
            length: rng.gen_range(10., 40.),
            root: Branch::new(0.2, Vector3::new(0., 0., 1.), &mut rng)
        }
    }

    pub fn build_mesh(&self) -> Result<Mesh, ChunkError> {
        let buffer = self.root.build_buffer(16);
        let mesh = Mesh::try_from(buffer)?;
        Ok(mesh)
    }
}
